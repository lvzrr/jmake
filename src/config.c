#include "jmake.h"

int	check_conf_exists(char *conf, t_settings *set)
{
	int	fd;

	if (!conf)
	{
		populate_default_config(set);
		return (-1);
	}
	if ((fd = open(conf, O_RDONLY)) < 0)
	{
		populate_default_config(set);
		return (-1);
	}
	close (fd);
	return (1);
}

void	populate_default_config(t_settings *set)
{
	set->doc_path = strdup(DOCS);
	set->src_path = strdup(SRC);
	set->bin_path = strdup(CLASSES);
	set->lib_path = strdup(LIB);
	set->pkg_path = strdup(PKG);

	if (!set->doc_path || !set->src_path || !set->bin_path ||
		!set->lib_path || !set->pkg_path)
	{
		free(set->doc_path);
		free(set->src_path);
		free(set->bin_path);
		free(set->lib_path);
		free(set->pkg_path);
		fprintf(stderr, "Error: Memory allocation failed in populate_default_config\n");
		exit(EXIT_FAILURE);
	}
}

char	*read_custom_file(char *fname)
{
	int				fd;
	unsigned int	len;
	char			buf[1];
	char			*fcontent;

	len = 0;
	fd = open(fname, O_RDONLY);
	if (fd < 0) return (NULL);
	while (read(fd, buf, 1) > 0)
		len++;
	if (len == 0)
	{
		close(fd);
		return (NULL);
	}
	close(fd);
	fd = open(fname, O_RDONLY);
	if (fd < 0) return (NULL);
	fcontent = (char *)malloc(len + 1);
	if (!fcontent)
	{
		close(fd);
		return (NULL);
	}
	if (read(fd, fcontent, len) != (ssize_t)len)
	{
		close(fd);
		free(fcontent);
		return (NULL);
	}
	fcontent[len] = 0;
	close(fd);
	return (fcontent);
}
