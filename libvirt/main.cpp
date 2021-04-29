#include <iostream>
#include <iomanip>
#include <numeric>
#include <string.h>

#include <libvirt/libvirt.h>

void list_connection_domains(virConnectPtr);
void print_domain(virDomainPtr);
void state_to_str(int, char*);
int is_valid_id(int);

// Connect to QEMU and list domains
int main() {
    const char* uri = "qemu:///system";
    virConnectPtr conn = virConnectOpen(uri);

    if (conn) {
        list_connection_domains(conn);
        virConnectClose(conn);
    } else {
        std::cout << "Could not connect\n";
    }

    return 0;
}

// List domains like 'virsh list --all' does
void list_connection_domains(virConnectPtr conn) {
    virDomainPtr* domains;
    int num_domains = virConnectListAllDomains(conn, &domains, 0);

    std::cout << std::left
            << std::setw(6) << "ID"
            << std::setw(16) << "Name"
            << std::setw(10) << "State"
            << "\n-------------------------------\n";

    for (size_t i = 0; i < num_domains; i++) {
        print_domain(domains[i]);
        virDomainFree(domains[i]);
    }
}

// Print line with domain info
void print_domain(virDomainPtr domain) {
    unsigned int id = virDomainGetID(domain);

    const char* name = virDomainGetName(domain);

    // Get state and its string representation
    int* state = (int*) malloc(sizeof(int));
    virDomainGetState(domain, state, NULL, 0);
    char* state_str = (char*) malloc(sizeof(char) * 20);
    state_to_str(*state, state_str);

    // Show id as "-" in case domain is not running
    std::string id_str = is_valid_id(*state) ? std::to_string(id) : "-";

    // Print domain info
    std::cout << std::left << std::setw(6) << id_str
        << std::setw(16) << name
        << std::setw(20) << state_str << "\n";

    free(state);
    free(state_str);
}

// Only showing "running" and "shut off" for simplicity
void state_to_str(int state, char* state_str) {
    switch (state) {
        case VIR_DOMAIN_RUNNING: {
            strcpy(state_str, "running");
            break;
        }
        case VIR_DOMAIN_SHUTOFF: {
            strcpy(state_str, "shut off");
            break;
        }
        default: {
            strcpy(state_str, "-");
        }
    }
}

int is_valid_id(int state) {
    return state == VIR_DOMAIN_RUNNING;
}