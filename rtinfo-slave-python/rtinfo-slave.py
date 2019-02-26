import requests
import redis
import time
import json

RTINFO_ENDPOINT = "http://clea.maxux.net:8089/json"

# This class is in another file for reusability
class DashboardSlave():
    def __init__(self, name):
        self.name = name
        self.redis = redis.Redis()
        self.payload = {}

    def publish(self):
        self.redis.publish("dashboard", json.dumps({"id": self.name, "payload": self.payload}))

    def sleep(self, seconds):
        time.sleep(seconds)

slave = DashboardSlave("rtinfo")

while True:
    print("[+] rtinfo: fetching")

    try:
        response = requests.get(RTINFO_ENDPOINT)
        slave.payload = response.json()

        print("[+] rtinfo: %d hosts found" % len(slave.payload['rtinfo']))

        slave.publish()

    except Exception as e:
        print(e)

    slave.sleep(1)
