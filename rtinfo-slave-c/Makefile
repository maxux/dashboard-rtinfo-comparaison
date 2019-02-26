EXEC = rtinfo-slave
SRC = $(wildcard *.c)
OBJ = $(SRC:.c=.o)

CFLAGS += -g -std=gnu99 -O2 -W -Wall
LDFLAGS += -Wl,-Bstatic -ljansson -lhiredis -Wl,-Bdynamic -lpthread -lrt -ldl -lcurl

all: $(EXEC)

$(EXEC): $(OBJ)
	$(CC) -o $@ $^ $(LDFLAGS)

%.o: %.c
	$(CC) $(CFLAGS) -c $<

clean:
	$(RM) *.o

mrproper: clean
	$(RM) $(EXEC)

