# rename `template` to name of project
TARGET=append_digit_primes
CXX=g++
OFILES=../common/tree.o ../common/util.o
LIBS=
CXXFLAGS=-g

$(TARGET): $(OFILES) $(TARGET).o
	$(CXX) $(CXXFLAGS) -o $@ $(TARGET).o $(OFILES) $(LIBS)

%.o: %.cpp
	$(CXX) $(CXXFLAGS) -c -o $@ $<

PHONY: clean run all

clean:
	rm *.o $(TARGET)

run:
	./$(TARGET)

all:
	make $(TARGET)
