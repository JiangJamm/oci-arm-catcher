#!/usr/bin/env python3
"""A tiny stand-in for the OCI API, for recording the demo GIFs.

Answers the calls oci-arm-catcher makes. LaunchInstance fails with
"Out of host capacity" MOCK_FAILS times (default 2), then succeeds.
It does not check signatures. Not for anything but demos.

    python3 assets/demo/mock-oci.py 8799
"""

import json
import os
import sys
from http.server import BaseHTTPRequestHandler, HTTPServer

FAILS = int(os.environ.get("MOCK_FAILS", "2"))
REGION = "us-ashburn-1"
ADS = [f"Abcd:US-ASHBURN-AD-{n}" for n in (1, 2, 3)]
attempts = 0


class Handler(BaseHTTPRequestHandler):
    def reply(self, status, body):
        data = json.dumps(body).encode()
        self.send_response(status)
        self.send_header("content-type", "application/json")
        self.send_header("content-length", str(len(data)))
        self.send_header("opc-request-id", "demo")
        self.end_headers()
        self.wfile.write(data)

    def do_GET(self):
        path = self.path.split("?")[0]
        if path.endswith("/availabilityDomains"):
            self.reply(200, [{"name": ad} for ad in ADS])
        elif path.endswith("/subnets"):
            self.reply(200, [{
                "id": f"ocid1.subnet.oc1.{REGION}.aaaademopublicsubnet",
                "displayName": "public-subnet",
                "cidrBlock": "10.0.0.0/24",
                "prohibitPublicIpOnVnic": False,
            }])
        elif path.endswith("/images"):
            self.reply(200, [{
                "id": f"ocid1.image.oc1.{REGION}.aaaademoubuntu2404arm",
                "displayName": "Canonical-Ubuntu-24.04-aarch64-2026.10.01-0",
            }])
        else:
            self.reply(404, {"code": "NotFound", "message": path})

    def do_POST(self):
        global attempts
        self.rfile.read(int(self.headers.get("content-length", 0)))
        attempts += 1
        if attempts <= FAILS:
            self.reply(500, {"code": "InternalError", "message": "Out of host capacity."})
        else:
            self.reply(200, {
                "id": f"ocid1.instance.oc1.{REGION}.aaaaarmfree",
                "displayName": "arm-free-1",
                "lifecycleState": "PROVISIONING",
                "region": REGION,
            })

    def log_message(self, *args):
        pass


if __name__ == "__main__":
    port = int(sys.argv[1]) if len(sys.argv) > 1 else 8799
    HTTPServer(("127.0.0.1", port), Handler).serve_forever()
