MEMORY {
  FLASH : ORIGIN = 0x08000000, LENGTH = 64000K
  RAM : ORIGIN = 0x20000000, LENGTH = 20000K
}

SECTIONS {
  .text : {
    *(.text.*); /* Place all .text input sections here */
  } > FLASH
  /* ... other sections like .data, .bss, etc. ... */
}