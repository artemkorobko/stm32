# FDU device implementation

This is the implementation of DFU device controlled by USB CDC ACM for STM32F103 MCU.

# Protocol
## Inbound version
Request firmware version information from MCU. Host should read outbound version response after sending this request. Below is the example representation of the request:
| Opcode |
| --- |
| 0x00 |

## Outbound version
Response with firmware version information from MCU. Below is the example representation of the response:
| Opcode | Major | Minor | Patch |
| --- | --- | --- | --- |
| 0x00 | 0x00 | 0x01 | 0x00 |

## Inbound device id
Request device id information from MCU. Host should read outbound device id response after sending this request. Below is the example representation of the request:
| Opcode |
| --- |
| 0x01 |

## Outbound device id
Response with device id information from MCU. Below is the example representation of the response:
| Opcode | ID_0 | ID_1 | ID_2 | ID_3 |
| --- | --- | --- | --- |  -- |
| 0x01 | 0x3B1A | 0x0701 | 0x32124353 | 0x354B4E00 |
