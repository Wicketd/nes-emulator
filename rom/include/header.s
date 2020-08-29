.pushseg
.segment "HEADER"

.byte "NES", $1A
.byte $02 ; PRG ROM chunks
.byte $01 ; CHR ROM chunks

.popseg
