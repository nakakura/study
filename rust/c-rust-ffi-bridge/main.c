#include <stdio.h>
#include <sys/types.h>
#include <dlfcn.h>

struct callbacks {
	int a;
	void (* const event)(void);
};

typedef void* create_p(void);

struct Telexistence{
	int a;
	void (* const destroy)(void);
	void (* const callback)(struct callbacks*);
};

typedef struct Telexistence* FUNCPTR();

void event(){
	printf("called from rust\n");
}

int main(){
	void *plugin = dlopen("./target/debug/libjanus_telexistence.so", RTLD_LOCAL | RTLD_LAZY);
	if (!plugin) {
		printf("couldn't load\n");
	}
	FUNCPTR *create = (FUNCPTR*) dlsym(plugin, "create");
	struct Telexistence *tex = create();
	printf("plugin loaded %d\n", tex->a);
	tex->destroy();
	struct callbacks cb = {
		.a = -1,
		.event = event
	};
	tex->callback(&cb);
}

