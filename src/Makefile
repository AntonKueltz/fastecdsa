CC = gcc
CFLAGS = -std=c99 -O2
LFLAGS = -lgmp
MAIN = ecdsa

test: test.o curveMath.o curve.o point.o
	$(CC) $(CFLAGS) -o $(MAIN) test.o curveMath.o curve.o point.o $(LFLAGS)
test.o: test.c curveMath.h
	$(CC) $(CFLAGS) -c test.c
curveMath.o: curveMath.c curveMath.h point.h curve.h
	$(CC) $(CFLAGS) -c curveMath.c
curve.o: curve.c curve.h
	$(CC) $(CFLAGS) -c curve.c
point.o: point.c point.h
	$(CC) $(CFLAGS) -c point.c
clean:
	$(RM) $(MAIN) *.o
