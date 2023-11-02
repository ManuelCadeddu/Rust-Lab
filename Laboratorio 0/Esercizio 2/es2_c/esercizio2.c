#include <stdio.h>
#include <stdlib.h>
#include <string.h>

#define N_ELEMENTS 100

typedef struct {
    int type;
    float val;
    long timestamp;

} ValueStruct;      // 12 byte

typedef struct {
    int type;
    float val[10];
    long timestamp;

} MValueStruct;     // 48 byte

typedef struct {
    int type;
    char message[21]; // stringa null terminated lung max 20

} MessageStruct;    // 28 byte

typedef struct {
    int type;
    union {
        ValueStruct val;
        MValueStruct mvals;
        MessageStruct messages;
    };
} ExportData;   // 52 byte

void export(ExportData *data, int n, FILE *pf) {

    fwrite(data, sizeof(ExportData), N_ELEMENTS, pf);
}

int main(){
    
    FILE *fp = fopen("file_es2", "wb");
    ValueStruct vs[N_ELEMENTS/4];
    MValueStruct mvs[N_ELEMENTS/4];
    MessageStruct ms[N_ELEMENTS/2];
    ExportData ed[N_ELEMENTS];

    /*
    printf("sizeof(int): %d\n", sizeof(int));
    printf("sizeof(float): %d\n", sizeof(float));
    printf("sizeof(long): %d\n", sizeof(long));
    printf("sizeof(char): %d\n", sizeof(char));
    printf("sizeof(ValueStruct): %d\n", sizeof(ValueStruct));
    printf("sizeof(MValueStruct: %d\n", sizeof(MValueStruct));
    printf("sizeof(MessageStruct): %d\n", sizeof(MessageStruct));
    printf("sizeof(ExportData): %d\n", sizeof(ExportData));
    */

    if ( fp == NULL ) {

        fprintf(stderr, "Error opening the file \"file_es2\"");
        return -1;
    }

    for ( int i = 0; i < N_ELEMENTS; i++ ){

        if( i < N_ELEMENTS/4 ){

            vs[i].type = 1;
            vs[i].val = i;
            vs[i].timestamp = i;

            ed[i].type = 1;
            ed[i].val = vs[i];
        
        } else if ( i >= N_ELEMENTS/4 && i < N_ELEMENTS/2 ) {

            int j = i - N_ELEMENTS/4;
            mvs[j].type = 2;

            for ( int k=0; k<10; k++ )
                mvs[j].val[k] = k;

            mvs[j].timestamp = i;

            ed[i].type = 2;
            ed[i].mvals = mvs[j];
        
        } else {

            int j = i - N_ELEMENTS/2;

            ms[j].type = 3;
            sprintf(ms[j].message, "ciao                ");

            ed[i].type = 3;
            ed[i].messages = ms[j];
        }
    }

    export(ed, N_ELEMENTS, fp);

    fclose(fp);

    return 0;
}