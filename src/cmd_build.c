#include "jmake.h"

void	free_cmd(t_cmd *cmd)
{
	int	i;

	if (!cmd)
		return ;
	i = 0;
	while (cmd->files && cmd->files[i])
		free(cmd->files[i++]);
	if (cmd->files)		free(cmd->files);
	i = 0;
	while (cmd->flags && cmd->flags[i])
		free(cmd->flags[i++]);
	if (cmd->flags)		free(cmd->flags);
	if (cmd->build_cmd)	free(cmd->build_cmd);
	if (cmd->cmd) 		free(cmd->cmd);
	if (cmd->classpath)	free(cmd->classpath);
}

char	*alloc_build_cmd(t_cmd *cmd)
{
	unsigned int	l;
	int				i;

	l = 1;
	i = 0;
	while (cmd->files[i])
		l += strlen(cmd->files[i++]) + 1;
	i = 0;
	while (cmd->flags[i])
		l += strlen(cmd->flags[i++]) + 1;
	l += strlen(cmd->classpath) + strlen(cmd->cmd) + 2;
	return ((char *)malloc(l * sizeof(char)));
}

t_cmd	*build_cmd(char *ex, char *classpath, char **files, char **flags)
{
	t_cmd	*cmd;

	cmd = (t_cmd *)malloc(sizeof(t_cmd));
	if (!cmd) return (NULL);
	cmd->cmd		=	ex;
	cmd->classpath	=	classpath;
	cmd->files		=	files;
	cmd->flags		=	flags;
	cmd->build_cmd	=	alloc_build_cmd(cmd);
	if (!cmd->build_cmd)
	{
		free_cmd(cmd);
		return (NULL);
	}
	return (cmd);
}
