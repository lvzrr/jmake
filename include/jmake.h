#ifndef JMAKE_H
# define JMAKE_H

#include <stdio.h>
#include <unistd.h>
#include <fcntl.h>
#include <stdlib.h>
#include <string.h>

# define DOCS			"./docs"
# define CLASSES		"./bin"
# define LIB			"./lib"
# define PKG			"./pkg"
# define SRC			"./src"
# define CONF			"./jmake.conf"

typedef	struct	s_settings
{
	char	*doc_path;
	char	*src_path;
	char	*bin_path;
	char	*lib_path;
	char	*pkg_path;
}	t_settings;

typedef struct	s_cmd
{
	char	*cmd;
	char	*classpath;
	char	**files;
	char	**flags;
	char	*build_cmd;
}	t_cmd;

void	run_cmd(t_cmd *cmd);
void	free_cmd(t_cmd *cmd);

#endif
