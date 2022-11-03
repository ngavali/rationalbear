package main

import (
	"bytes"
	"encoding/binary"
	"fmt"
	"log"
	"net"
)

const (
	HEADERLENGTH             = 12
	DNSRESOURCENAMEMAXLENGTH = 255
	QUESTIONQUERYINFOLENGTH  = 4

	DNSHeaderFormatterText = `DNSHeader {
 IDentification : %d
 Flags         	: %016b
 Questions      : %d
 AnswerRRs      : %d
 AuthorityRRs   : %d
 AdditionalRRs  : %d
}`

	QuestionFormatterText = `Question {
 Name     : [len=%d] %s
 Query    : %v
}`

	QueryFormatterText = `Query {
 Type     : %d
 Class    : %d
}`

	AnswerFormatterText = `Answer {
 Name     : [len=%d] %s
 Query    : %v
 TTL      : %d
 RdLength : %d
 RData    : [len=%d] %s
}`
)

var (
	OWNEDDOMAINS = map[string]int32{
		"www.google.com": 84734,
	}
	NETWORKORDER = binary.BigEndian
)

func dumpData(data []byte) {
	log.Printf("!!!DEBUG!!! Data: length=%d bytes...", len(data))
	for i, c := range data {
		if i%16 == 0 {
			fmt.Println("")
		}
		fmt.Printf("%02x", c)
		if i%2 == 1 {
			fmt.Printf(" ")
		}
	}
	fmt.Println("")
	log.Println("!!!DEBUG!!!")
}

type DNSService struct {
	shutdown bool
}

type Service interface {
	Run()
}

type DNSHeader struct {
	IDentification uint16
	Flags          uint16
	Questions      uint16
	AnswerRRs      uint16
	AuthorityRRs   uint16
	AdditionalRRs  uint16
}

func (h DNSHeader) String() string {
	return fmt.Sprintf(DNSHeaderFormatterText, h.IDentification, h.Flags, h.Questions, h.AnswerRRs, h.AuthorityRRs, h.AdditionalRRs)
}

type Query struct {
	Type  uint16
	Class uint16
}

func (q Query) String() string {
	return fmt.Sprintf(QueryFormatterText, q.Type, q.Class)
}

type Question struct {
	Name []byte
	Query
}

func (q Question) String() string {
	return fmt.Sprintf(QuestionFormatterText, len(q.Name), q.Name, q.Query)
}

type Answer struct {
	Name []byte
	Query
	TTL      uint32
	RdLength uint16
	RData    []byte
}

func (a Answer) String() string {
	return fmt.Sprintf(AnswerFormatterText, len(a.Name), a.Name, a.Query, a.TTL, a.RdLength, len(a.RData), a.RData)
}

type DNSRequest struct {
	DNSHeader
	Question
}

func NewDNSRequest(
	data []byte,
) *DNSRequest {

	msgSize := len(data)
	dnsRequest := DNSRequest{}
	binary.Read(bytes.NewBuffer(data[:HEADERLENGTH]), NETWORKORDER, &dnsRequest.DNSHeader)
	log.Printf("Request: %+v\n", dnsRequest.DNSHeader)
	dumpData(data[:msgSize])

	dnsQuestion := data[HEADERLENGTH:]
	dnsResourceName := make([]byte, 0, DNSRESOURCENAMEMAXLENGTH)

	var k, resourceNameLength, questionLength = 0, 0, HEADERLENGTH

	//Process Questions
	//Actually, Questions is always = 1
	for i := 0; i < int(dnsRequest.DNSHeader.Questions); i++ {
		//Process resource record
		for dnsQuestion[k] != 0 {
			resourceNameLength = k + int(dnsQuestion[k]) + 1
			k++
			dnsResourceName = append(append(dnsResourceName,
				dnsQuestion[k:resourceNameLength]...),
				'.',
			)
			k = resourceNameLength
		}
		k++

		query := Query{}
		queryData := bytes.NewBuffer(dnsQuestion[k : k+QUESTIONQUERYINFOLENGTH])
		binary.Read(queryData, NETWORKORDER, &query)
		dnsRequest.Question = Question{
			dnsQuestion[:resourceNameLength+1],
			query,
		}

		questionLength += k + QUESTIONQUERYINFOLENGTH

		//log.Printf("Looking up for : %s", dnsResourceName)

	}
	//We are not worried about AdditionalRRs here
	dnsRequest.DNSHeader.AdditionalRRs = 0

	return &dnsRequest

}

type DNSResponse struct {
	DNSHeader
	Question
	Answer
}

func NewDNSResponse() *DNSResponse {
	return &DNSResponse{}
}

func (r DNSResponse) IntoBytesBuffer(
	dnsResponseHeader *bytes.Buffer,
) {
	//Write Header
	binary.Write(dnsResponseHeader, NETWORKORDER, &r.DNSHeader)

	//Write Question
	binary.Write(dnsResponseHeader, NETWORKORDER, &r.Question.Name)
	binary.Write(dnsResponseHeader, NETWORKORDER, &r.Question.Query)

	//Write Answer
	binary.Write(dnsResponseHeader, NETWORKORDER, &r.Answer.Name)
	binary.Write(dnsResponseHeader, NETWORKORDER, &r.Answer.Query)
	binary.Write(dnsResponseHeader, NETWORKORDER, &r.Answer.TTL)
	binary.Write(dnsResponseHeader, NETWORKORDER, &r.Answer.RdLength)
	binary.Write(dnsResponseHeader, NETWORKORDER, &r.Answer.RData)

	dumpData(dnsResponseHeader.Bytes())
}

func (s *DNSService) HandleRequest(udpConnection *net.UDPConn, addr *net.UDPAddr, data []byte) {

	log.Printf("Requestor : %+v", addr)
	dnsRequest := NewDNSRequest(data)
	//Use header from the request and update it for the response
	log.Printf("Request details: %+v\n", dnsRequest)

	if dnsRequest.DNSHeader.Flags&1<<15 == 0 { //It is a question packet

		dnsResponse := NewDNSResponse()
		dnsResponse.DNSHeader = dnsRequest.DNSHeader
		//Set type as answer
		dnsResponse.DNSHeader.Flags |= 1 << 15
		dnsResponse.Question = dnsRequest.Question
		dnsResponse.DNSHeader.AnswerRRs = 1

		dnsResponse.Answer = Answer{
			dnsRequest.Question.Name,
			dnsRequest.Question.Query,
			60,
			4,
			[]byte{1, 1, 1, 1}, //You can put any valid response here !!!
		}

		log.Printf("Response details: %+v\n", dnsResponse)

		dnsResponseData := new(bytes.Buffer)
		dnsResponse.IntoBytesBuffer(dnsResponseData)
		udpConnection.WriteToUDP(dnsResponseData.Bytes(), addr)
	}

}

func (s *DNSService) Run() {

	udpAddr, _err := net.ResolveUDPAddr("udp", "0.0.0.0:53")
	if _err != nil {
		log.Printf("net.ResolveUDPAddr: %s", _err)
	}
	udpConnection, _err := net.ListenUDP("udp", udpAddr)
	if _err != nil {
		log.Printf("net.ListenUDP: %s", _err)
	}
	defer udpConnection.Close()

	data := make([]byte, 1024)

	for {

		msgSize, addr, _err := udpConnection.ReadFromUDP(data)
		if _err != nil || msgSize == 0 {
			log.Printf("udpConnection.Read: %s", _err)
			break
		}

		s.HandleRequest(udpConnection, addr, data[:msgSize])

	}
}

func main() {
	dnsService := DNSService{}
	dnsService.Run()
}
