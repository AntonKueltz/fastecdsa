CC = gcc
CFLAGS = -std=c99 -O2
LFLAGS = -lgmp -lpython2.7
MAIN = ecdsa

test: test.o _ecdsa.o curveMath.o curve.o point.o
	$(CC) $(CFLAGS) -o $(MAIN) test.o _ecdsa.o curveMath.o curve.o point.o $(LFLAGS)
test.o: test.c curveMath.h
	$(CC) $(CFLAGS) -c test.c
_ecdsa.o: _ecdsa.c _ecdsa.h curveMath.h
	$(CC) $(CFLAGS) -c _ecdsa.c
curveMath.o: curveMath.c curveMath.h point.h curve.h
	$(CC) $(CFLAGS) -c curveMath.c
curve.o: curve.c curve.h
	$(CC) $(CFLAGS) -c curve.c
point.o: point.c point.h
	$(CC) $(CFLAGS) -c point.c
clean:
	$(RM) $(MAIN) *.o
