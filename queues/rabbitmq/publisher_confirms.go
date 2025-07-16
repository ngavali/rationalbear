package main

import (
	"fmt"
	"log"
	"math/rand"
	"os"
	"strconv"
	"sync"
	"time"

	amqp "github.com/streadway/amqp"
)

type Message struct {
	id       uint64
	content  string
	priority uint8
}

func failOnError(_err error, msg string) {
	if _err != nil {
		log.Fatalf("%s : %s", msg, _err)
	}
}

func main() {

	mymessage := os.Args[1]
	batchSize := os.Args[2]
	//mpriority := os.Args[2]

	//Get a connection
	conn, _err := amqp.Dial("amqp://etl_user:etl_pass@localhost:5672/etl")
	failOnError(_err, "Failed to connect to RabbitMQ")
	defer conn.Close()

	//Create a channel
	ch, _err := conn.Channel()
	failOnError(_err, "Failed to open a channel")
	defer ch.Close()

	fmt.Printf("%+v\n", ch)

	_err = ch.Confirm(false)
	failOnError(_err, "Cannot set channel in confirm mode")

	var wg = &sync.WaitGroup{}
	//	var confirm = make(chan amqp.Confirmation, 10)

	Confirmation := ch.NotifyPublish(make(chan amqp.Confirmation, 1))

	//Create a queue
	q, _err := ch.QueueDeclare(
		"jobs", // name
		true,   // durable
		false,  // delete when unused
		false,  // exclusive
		false,  // no-wait
		map[string]interface{}{"x-max-priority": 20},
		//		map[string]interface{}{"x-max-length": 2, "x-overflow": "reject-publish"}, // arguments
	)

	failOnError(_err, "Failed to declare a queue")

	fmt.Printf("Queue: %+v\n", q)

	time.Sleep(1 * time.Second)

	/*
		go func() {
			for C := range Confirmation {
				log.Printf("Confirmation : %v", C)
			}
			wg.Done()
		}()
	*/
	message_num, _err := strconv.Atoi(mymessage)

	var BatchSize, DeliveryTag uint64 = 100, 0
	b, _err := strconv.Atoi(batchSize)
	if _err == nil {
		BatchSize = uint64(b)
	}
	fmt.Println("BatchSize =", BatchSize)

	//var done = make(chan uint64)
	var messageChan = make(chan Message, BatchSize)
	var messageToConfirmChan = make(chan Message, BatchSize)
	//var res = make(chan uint64)

	handleConfirmation := func(message <-chan Message) {

		for {
			if msg, ok := <-message; ok {
				C, _err := <-Confirmation
				log.Printf("Confirmation : %+v, Error : %v :: Message : %+v", C, _err, msg)
				if C.Ack == false {
					fmt.Println("Resending message:", msg)
					messageChan <- msg
					fmt.Println("Resending message done:")
				}
			} else {
				log.Printf("All messages processed.")
				break
			}
		}

		log.Printf("Closing confirmation channel.")
		wg.Done()
	}

	handlePublish := func(message <-chan Message) {

		for {
			if msg, ok := <-message; ok {
				DeliveryTag++
				msg.id = DeliveryTag
				fmt.Printf("received message to publish: %+v\n", msg)
				_err = ch.Publish(
					"",     //exchange
					q.Name, // routing key
					false,  //mandatory
					false,  //immediate
					amqp.Publishing{
						ContentType:  "text/plain",
						Body:         []byte(msg.content),
						DeliveryMode: amqp.Persistent,
						Priority:     msg.priority,
						AppId:        "jobScheduler",
					})
				failOnError(_err, "Failed to publish a message")
				fmt.Printf("Message published. %+v\n", DeliveryTag)
				messageToConfirmChan <- msg
				fmt.Println("Message send to confirm channel")
			} else {
				break
			}
		}

		log.Printf("Published all received messages.")
		wg.Done()
	}

	/*
		requeuer := func(message <-chan Message) {
			for msg := range message {
				messageChan <- msg
				messageToConfirmChan <- msg
			}
		}
	*/
	wg.Add(1)
	go handlePublish(messageChan)

	wg.Add(1)
	go handleConfirmation(messageToConfirmChan)

	for i := range make([]int, message_num) {
		//A message to send

		priority := uint8(rand.Intn(10))
		//priority, _ := strconv.Atoi(mpriority)

		body := fmt.Sprintf("node-3-JOB-%d priority=%d", i+1, priority)
		fmt.Printf("Sending message: %v, priority=%d\n", i+1, priority)
		//Publish the message

		messageChan <- Message{content: body, priority: priority}
		fmt.Printf("Message sent: %v\n", i+1)

	}

	fmt.Println("I am done!!!")

	fmt.Printf("Queue: %+v\n", q)

	//close(messageChan)
	//close(messageToConfirmChan)

	wg.Wait()

}
