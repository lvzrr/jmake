COMPILER	:= cc
FLAGS		:= -Wall -Wextra -Werror -O3 -I./include

NAME		:= jmake

SRCDIR		:= src
INCLUDE		:= include

all:
	$(COMPILER) $(SRCDIR)/*.c $(FLAGS) -o $(NAME)
clean:
	rm $(name)
.PHONY:
	clean all
