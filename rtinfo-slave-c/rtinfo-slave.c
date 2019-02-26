#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include <jansson.h>
#include <unistd.h>
#include <curl/curl.h>
#include <hiredis/hiredis.h>

typedef struct memory_t {
    char *memory;
    size_t size;

} memory_t;

#define RTINFO_ENDPOINT "http://clea.maxux.net:8089/json"
#define REDIS_HOST      "localhost"
#define REDIS_PORT      6379

void diep(char *str) {
    perror(str);
    exit(EXIT_FAILURE);
}

redisContext *redis_init() {
    redisContext *ctx;

    if(!(ctx = redisConnect(REDIS_HOST, REDIS_PORT)))
        diep("redisConnect");

    if(ctx->err) {
        printf("[-] redis: %s\n", ctx->errstr);
        exit(EXIT_FAILURE);
    }

    return ctx;
}

static size_t curl_into_memory(void *contents, size_t size, size_t nmemb, void *userp) {
    size_t realsize = size * nmemb;
    memory_t *mem = (memory_t *) userp;

    if(!(mem->memory = realloc(mem->memory, mem->size + realsize + 1)))
        diep("realloc");

    memcpy(&(mem->memory[mem->size]), contents, realsize);
    mem->size += realsize;
    mem->memory[mem->size] = 0;

    return realsize;
}

char *download() {
    CURL *curl;
    CURLcode res;

    memory_t chunk = {
        .memory = NULL,
        .size = 0,
    };

    curl = curl_easy_init();
    curl_easy_setopt(curl, CURLOPT_URL, RTINFO_ENDPOINT);
    curl_easy_setopt(curl, CURLOPT_WRITEFUNCTION, curl_into_memory);
    curl_easy_setopt(curl, CURLOPT_WRITEDATA, (void *) &chunk);

    if((res = curl_easy_perform(curl)) != CURLE_OK) {
        fprintf(stderr, "download failed: %s\n", curl_easy_strerror(res));
        exit(EXIT_FAILURE);
    }

    curl_easy_cleanup(curl);

    return chunk.memory;
}

char *convert(char *source) {
    json_t *root;
    json_error_t error;

    if(!(root = json_loads(source, 0, &error))) {
        fprintf(stderr, "json error: on line %d: %s\n", error.line, error.text);
        return NULL;
    }

    int hosts = json_array_size(json_object_get(root, "rtinfo"));
    printf("[+] rtinfo: %d hosts found\n", hosts);

    json_t *converted = json_object();
    json_object_set(converted, "id", json_string("rtinfo"));
    json_object_set(converted, "payload", root);

    char *result = json_dumps(converted, 0);
    json_decref(converted);
    json_decref(root);

    return result;
}

int publish(redisContext *redis, char *payload) {
    redisReply *reply;

    if(!(reply = redisCommand(redis, "PUBLISH dashboard %s", payload)))
        diep("redisCommand");

    return 0;
}

int main(void) {
    char *request = NULL;
    char *final = NULL;
    redisContext *redis = NULL;

    curl_global_init(CURL_GLOBAL_ALL);

    redis = redis_init();

    while(1) {
        printf("[+] rtinfo: fetching...\n");

        if(!(request = download())) {
            fprintf(stderr, "[-] could not download target\n");
            exit(EXIT_FAILURE);
        }

        if(!(final = convert(request))) {
            fprintf(stderr, "[-] could not convert json\n");
            exit(EXIT_FAILURE);
        }

        publish(redis, final);

        free(request);
        free(final);

        usleep(1000000);
    }

    return 0;
}
