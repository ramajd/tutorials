CC = gcc
CFLAGS = -Wall -O3 `pkg-config --cflags gtk4`
LIBS = `pkg-config --libs gtk4`


TARGET = hello_gtk

SRC = hello_gtk.c
OBJ = hello_gtk.o

all: $(TARGET)

$(TARGET): $(OBJ)
	$(CC) $(CFLAGS) $(OBJ) -o $(TARGET) $(LIBS)

$(OBJ): hello_gtk.c
	$(CC) $(CFLAGS) -c hello_gtk.c -o hello_gtk.o

clean:
	rm -f $(OBJ) $(TARGET)

.PHONY: all clean