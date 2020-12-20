#include <stdio.h>
#include <ctype.h>
#include <atmi.h>
#include <userlog.h>
#include <fml32.h>
#include <string.h>

void c_upper(TPSVCINFO *rqst)
{

	int i;
	FBFR32 *requestfml;
	FBFR32 *responsefml;
	char message[1024];

	requestfml = (FBFR32 *)rqst->data;


	if (Fget32(requestfml, Fldid32("MESSAGE"), 0, (char *) &message, NULL) == -1) {
                userlog("Error get data");
		tpreturn(TPFAIL, 0, rqst->data, 0L, 0);
	}
	userlog("Got [%s]",message);
	for(i = 0 ; i < strlen(message) ; i++) {
		message[i]=toupper(message[i]);
	}
	if ( NULL == (responsefml = (FBFR32 *) tpalloc("FML32",NULL,sizeof(message)))) {
                userlog("tpalloc failed");
		tpreturn(TPFAIL, 0, rqst->data, 0L, 0);
	}
	if ( Fadd32(responsefml, Fldid32("MESSAGE"),&message[0], (FLDLEN32)strlen(message)) == -1 ) {
                userlog("Fadd32 failed");
		tpreturn(TPFAIL, 0, rqst->data, 0L, 0);
	}

	tpreturn(TPSUCCESS, 0, (char *)responsefml, 0L, 0);

}
