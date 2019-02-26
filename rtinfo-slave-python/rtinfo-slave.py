import requests
import redis
import time
import json

RTINFO_ENDPOINT = "http://clea.maxux.net:8089/json"

r = redis.Redis()

while True:
    print("[+] rtinfo: fetching")

    try:
        response = requests.get(RTINFO_ENDPOINT)
        payload = response.json()

        print("[+] rtinfo: %d hosts found" % len(payload['rtinfo']))

        r.publish("dashboard", json.dumps({"id": "rtinfo", "payload": payload}))

    except Exception as e:
        print(e)

    time.sleep(1)
