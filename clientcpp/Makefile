
PROG_NAME = bin/multiscript

CC = gcc
CXX = g++
INCLUDEDIR = \
	-I./include \
	
LIBDIR = \

LIBS = \
	-lpthread \
	-lrt \

CXXFLAGS = -O0 -rdynamic -Wall -DLIBSSH_STATIC $(INCLUDEDIR)
CFLAGS = -O0 -rdynamic -Wall -DLIBSSH_STATIC $(INCLUDEFIR)
LDFLAGS = -m64 -rdynamic $(LIBDIR) $(LIBS)

MAKEFLAGS += --no-builtin-rules
.SUFFIXES:

CXXSOURCES = $(wildcard src/*.cpp)
CSOURCES = $(wildcard src/*.c)

all: $(PROG_NAME)

$(PROG_NAME): $(CXXSOURCES:.cpp=.o) $(CSOURCES:.c=.o)
	$(CXX) -o $@ $^ $(LDFLAGS)

%.o: %.cpp
	$(CXX) -c $< $(CXXFLAGS) -o $@

%.o: %.c
	$(CC) -c $< $(CFLAGS) -o $@

clean:
	-rm -f src/*.o $(PROG_NAME)

rebuild:
	$(MAKE) clean
	$(MAKE)

-include $(CXXSOURCES:.cpp=.d)
-include $(CSOURCES:.cpp=.d)

%.d: %.cpp
	$(CXX) $< -MM -MT '$*.o $*.d' -MF $*.d $(CXXFLAGS)

%.d: %.c
	$(CC) $< -MM -MT ’$*.o $*.d ’ -MF $*.d $(CFLAGS)

.PHONY:	clean all

