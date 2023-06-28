# Software Stack


Protocols: 
```
LAYER         PROTOCOL   JOB
----------------------------
application   DIM        Define message structure
application   AES        Encrypt/Decrypt message
application   TLS        Encrypt/Decrypt headers
presentation
session
transport     TCP        Attach TCP headers to tell network layer targets and sources
network       IPv 4/6    Transmit to the target destination over the net
```