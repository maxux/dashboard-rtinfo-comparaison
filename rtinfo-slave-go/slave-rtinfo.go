package main

import (
    "net/http"
    "io/ioutil"
    "fmt"
    "log"
    "encoding/json"
    "time"
    "github.com/go-redis/redis"
)

type Dashboard struct {
    Id string                      `json:"id"`
    Payload map[string]interface{} `json:"payload"`
}

const REDIS_TARGET = "localhost:6379"
const RTINFO_ENDPOINT = "http://clea.maxux.net:8089/json"

func download() map[string]interface{} {
    resp, err := http.Get(RTINFO_ENDPOINT)
    if err != nil {
	    log.Panic(err)
    }

    defer resp.Body.Close()
    body, err := ioutil.ReadAll(resp.Body)

    var parsed map[string]interface{}
    if err := json.Unmarshal(body, &parsed); err != nil {
        log.Panic(err);
    }

    rtinfo, ok := parsed["rtinfo"].([]interface{})
    if !ok {
        log.Panic("json parsing error");
    }

    fmt.Printf("[+] rtinfo: %d hosts found\n", len(rtinfo))

    return parsed
}

func publish(rtinfo map[string]interface{}, client *redis.Client) {
    encoded, err := json.Marshal(Dashboard{Id: "rtinfo", Payload: rtinfo})
    if err != nil {
        log.Panic(err)
    }

    err = client.Publish("dashboard", string(encoded)).Err()
	if err != nil {
		log.Panic(err)
	}


}

func main() {
    client := redis.NewClient(&redis.Options{Addr: REDIS_TARGET,})

    for {
        fmt.Println("[+] rtinfo: fetching")
        object := download()

        publish(object, client)

        time.Sleep(1 * time.Second)
    }
}
