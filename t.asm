
start:
		;move.l	123(a0,d0*2),a1
		;move.w	123(a1,d2*4),9876(a2,d3*2)
		move.w	-8(pc,d2*8),d7
