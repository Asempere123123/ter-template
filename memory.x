MEMORY
{
    FLASH : ORIGIN = {{ flash-begin }}, LENGTH = {{ flash-len }}
    RAM   : ORIGIN = 0x20000000, LENGTH =  {{ ram-len }}
}
