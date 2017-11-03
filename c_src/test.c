#include <stdio.h>
#include <stdlib.h>
#include <stdint.h>
#include <sys/ioctl.h>
#include <bsm/audit.h>
#include <security/audit/audit_ioctl.h>

int main (int argc, char *argv[]) {

    FILE* auditFile;
    int auditFileDescriptor;
    char* auditPipe = "/dev/auditpipe";


    auditFile = fopen(auditPipe, "r");
    if (auditFile == NULL) {
        fprintf(stderr, "Unable to open audit pipe: %s\n", auditPipe);
        perror("Error ");
        exit(1);
    }
    auditFileDescriptor = fileno(auditFile);


    int queueLength;
    int ioctlReturn = ioctl(
        auditFileDescriptor,
        AUDITPIPE_GET_QLIMIT_MAX,
        &queueLength);
    if (ioctlReturn == -1) {
        fprintf(stderr,
            "Unable to get the maximum queue length of the audit pipe.\n");
        perror("Error ");
    }

printf("Queue Length %d\n", queueLength);

exit(0);

}
