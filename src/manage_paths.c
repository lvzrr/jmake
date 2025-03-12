#include "jmake.h"

char	*join(char *p1, char *p2)
{
	char	*pout;
	int		len;
	int		needs_sl;

	needs_sl = (p1[strlen(p1) - 1] != '/');
	len = strlen(p1) + strlen(p2) + (needs_sl ? 2 : 1);
	pout = (char *)malloc(len);
	if (!pout)
		return (NULL);
	if (needs_sl)
		snprintf(pout, len, "%s/%s", p1, p2);
	else
		snprintf(pout, len, "%s%s", p1, p2);
	return (pout);
}

void	create_dir(char *dname)
{
	if (mkdir(dname, 0777) == -1 && errno != EEXIST)
	{
		fprintf(stderr, "Error creating dir %s\n", dname);
		exit(EXIT_FAILURE);
	}

}
