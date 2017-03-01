CC = gcc
CFLAGS = -O2 -Wall -Wstrict-prototypes
IFLAGS = -I/usr/include/python2.7
LFLAGS = -lgmp -lpython2.7
MAIN = ecdsa

test: test.o _ecdsa.o curveMath.o curve.o point.o
	$(CC) $(CFLAGS) $(IFLAGS) -o $(MAIN) test.o _ecdsa.o curveMath.o curve.o point.o $(LFLAGS)
test.o: test.c curveMath.h
	$(CC) $(CFLAGS) $(IFLAGS) -c test.c
_ecdsa.o: _ecdsa.c _ecdsa.h curveMath.h
	$(CC) $(CFLAGS) $(IFLAGS) -c _ecdsa.c
curveMath.o: curveMath.c curveMath.h point.h curve.h
	$(CC) $(CFLAGS) $(IFLAGS) -c curveMath.c
curve.o: curve.c curve.h
	$(CC) $(CFLAGS) $(IFLAGS) -c curve.c
point.o: point.c point.h
	$(CC) $(CFLAGS) $(IFLAGS) -c point.c
clean:
	$(RM) $(MAIN) *.o
