package main

import (
	"bytes"
	"encoding/binary"
	"fmt"
	"log"
	"net"
	"strings"
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
 RData    : %d
}`
)

var (
	OWNEDDOMAINS = map[string][]uint32{
		"www.example.com": {
			0xc0a8006d,
			0xc0a8006e,
		},
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

func (q Question) nameAsString() string {
	k := 0
	name := []string{}
	for q.Name[k] != 0 {
		length := k + int(q.Name[k]) + 1
		name = append(name, string(q.Name[k+1:length]))
		k = length
	}

	return strings.Join(name, ".")
}

type Answer struct {
	Name []byte
	Query
	TTL      uint32
	RdLength uint16
	RData    uint32 ///Always responding with A record //RData    []byte
}

func (a Answer) String() string {
	//return fmt.Sprintf(AnswerFormatterText, len(a.Name), a.Name, a.Query, a.TTL, a.RdLength, len(a.RData), a.RData) ///When using other types in response
	return fmt.Sprintf(AnswerFormatterText, len(a.Name), a.Name, a.Query, a.TTL, a.RdLength, a.RData)
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

	var k, questionLength = 0, HEADERLENGTH

	//Process Questions
	//Actually, Questions is always = 1
	for i := 0; i < int(dnsRequest.DNSHeader.Questions); i++ {
		//Process resource record
		for dnsQuestion[k] != 0 {
			k++
		}
		k++

		query := Query{}
		queryData := bytes.NewBuffer(dnsQuestion[k : k+QUESTIONQUERYINFOLENGTH])
		binary.Read(queryData, NETWORKORDER, &query)
		dnsRequest.Question = Question{
			dnsQuestion[:k],
			query,
		}

		questionLength += k + QUESTIONQUERYINFOLENGTH

		log.Printf("Looking up for : %s", dnsRequest.Question.nameAsString())

	}
	//We are not worried about AdditionalRRs here
	dnsRequest.DNSHeader.AdditionalRRs = 0

	return &dnsRequest

}

type DNSResponse struct {
	DNSHeader
	Question
	Answer []Answer
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

	for _, answer := range r.Answer {
		//Write Answer
		binary.Write(dnsResponseHeader, NETWORKORDER, &answer.Name)
		binary.Write(dnsResponseHeader, NETWORKORDER, &answer.Query)
		binary.Write(dnsResponseHeader, NETWORKORDER, &answer.TTL)
		binary.Write(dnsResponseHeader, NETWORKORDER, &answer.RdLength)
		binary.Write(dnsResponseHeader, NETWORKORDER, &answer.RData)
	}

	dumpData(dnsResponseHeader.Bytes())
}

func (s *DNSService) ProcessRequesst(dnsRequest *DNSRequest) *DNSResponse {

	if dnsRequest.DNSHeader.Flags&1<<15 == 0 { //It is a question packet

		dnsResponse := NewDNSResponse()
		dnsResponse.DNSHeader = dnsRequest.DNSHeader
		//Set type as answer
		dnsResponse.DNSHeader.Flags |= 1 << 15
		dnsResponse.Question = dnsRequest.Question

		if address, ok := OWNEDDOMAINS[dnsRequest.Question.nameAsString()]; ok {
			dnsResponse.DNSHeader.AnswerRRs = uint16(len(address))
			for _, ip := range address {
				dnsResponse.Answer = append(dnsResponse.Answer,
					Answer{
						dnsRequest.Question.Name,
						dnsRequest.Question.Query,
						60,
						4,
						ip, //You can put any valid response here !!!
					})
			}
		}
		return dnsResponse
	}
	return nil
}

func (s *DNSService) HandleRequest(udpConnection *net.UDPConn, msg UDPMessage) {
	//, addr *net.UDPAddr, data []byte) {

	log.Printf("Requestor : %+v", msg.UDPAddr)
	dnsRequest := NewDNSRequest(msg.data)
	//Use header from the request and update it for the response
	log.Printf("Request details: %+v\n", dnsRequest)

	if dnsResponse := s.ProcessRequesst(dnsRequest); dnsResponse != nil {

		log.Printf("Answering for : %s", dnsResponse.Question.nameAsString())
		log.Printf("Response details: %+v\n", dnsResponse)

		dnsResponseData := new(bytes.Buffer)
		dnsResponse.IntoBytesBuffer(dnsResponseData)
		udpConnection.WriteToUDP(dnsResponseData.Bytes(), msg.UDPAddr)
	} else {

		log.Printf("Couldn't handle this request. Bye!")
	}

}

type UDPMessage struct {
	*net.UDPAddr
	msg_len uint32
	data    []byte
	err     error
}

func (s *DNSService) ReadRequestIntoChan(udpConnection *net.UDPConn, msg chan<- UDPMessage) {
	data := make([]byte, 1024)
	for !s.shutdown {
		msgSize, addr, _err := udpConnection.ReadFromUDP(data)
		if _err != nil || msgSize == 0 {
			log.Printf("udpConnection.Read: %s", _err)
			s.shutdown = true
		} else {
			msg <- UDPMessage{
				addr,
				uint32(msgSize),
				data[:msgSize],
				_err,
			}
		}
	}
}

func (s *DNSService) Run() {

	c := signal_handler()

	udpAddr, _err := net.ResolveUDPAddr("udp", "0.0.0.0:53")
	if _err != nil {
		log.Printf("net.ResolveUDPAddr: %s", _err)
	}
	udpConnection, _err := net.ListenUDP("udp", udpAddr)
	if _err != nil {
		log.Printf("net.ListenUDP: %s", _err)
	}
	defer udpConnection.Close()

	msgChan := make(chan UDPMessage)

	go s.ReadRequestIntoChan(udpConnection, msgChan)

	for !s.shutdown {

		select {
		case msg := <-c:
			log.Printf("Exiting... %+v", msg)
			s.shutdown = true
			close(msgChan)
		case msg := <-msgChan:
			if msg.err != nil {
				log.Printf("Error reading the data. %+v", msg.err)
			} else {
				s.HandleRequest(udpConnection, msg)
			}
		}

	}
}

func main() {
	dnsService := DNSService{}
	dnsService.Run()
}
