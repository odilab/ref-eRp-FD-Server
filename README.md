# ARCHIVED
This project is no longer maintained and no more work will be done. The bootstrap phase of this reference implementation has come to an end. If you are interested in the productive systems source code, have a look at https://github.com/eRP-FD.

# Introduction

This project acts as reference implementation of main aspects of an e-prescription server designed by gematik.

Specifications of e-prescription application (E-Rezept, eRp) are published at
[Gematik Fachportal](https://fachportal.gematik.de/spezifikationen/online-produktivbetrieb/konzepte-und-spezifikationen/)

This implementation follows "Spezifikation E-Rezept Fachdienst" \[gemSpec\_FD\_eRP\] specification
available in specification bundle at location above. Additionally it follows further specifications
referenced by \[gemSpec\_FD\_eRP\].

This server offers a FHIR interface according to [HL7 FHIR](https://hl7.org/FHIR/) standard,
profiled to e-prescription needs. Profiling information are available at
[Simplifier](http://gematik.de/fhir/).

In order to run the server in a trusted execution environment (VAU), it also implements a separate
protocol on top of http (eRp VAU protocol).

To get an overview about the server API we recommend reading [API description](https://github.com/gematik/api-erp).

# Limitations

There are limitations to this implementation as currently some parts are not implemented.
Following functionality is available:

-   REST server basics

-   eRp VAU protocol encryption and decryption

-   FHIR server basics:

    -   Capability Statement generation

    -   XML and JSON serialization and deserialization

    -   \_format parameter handling for Capability Statement

-   FHIR resources and operations

    -   Task resource

        -   read interaction

        -   $create operation

        -   $activate operation

        -   $accept operation

        -   $reject operation

        -   $close operation

    -   Communication resource

        -   create interaction

        -   read interaction

        -   delete interaction

    -   AuditEvent

        -   read interaction

    -   Device Resource

        -   read interaction

-   access code generation

-   separate interfaces for eRp-App (FdV) and medical suppliers/pharmacies (LE)

-   Access token validation

-   Download and provide endpoints for TSL (Trust Status List)

There is no complete workflow implemented just now. It is intended as an very early release.

# License

Copyright (c) 2020 gematik GmbH

Licensed under the Apache License, Version 2.0 (the "License");
you may not use this file except in compliance with the License.
You may obtain a copy of the License at

<http://www.apache.org/licenses/LICENSE-2.0>

Unless required by applicable law or agreed to in writing, software
distributed under the License is distributed on an "AS IS" BASIS,
WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
See the License for the specific language governing permissions and
limitations under the License.

# Overview

E-prescription server is a central part of e-prescription application and acts as a backend service
of e-prescription App as well as a central service of practitioner’s medical practice
administration systems and pharmacy’s administration systems.

To get an overview following picture depicts the system context and some
inner components of e-prescription server.

![system context overview](doc/images/system_context_overview.png)

# Getting started

## Rust Toolchain

To build the ref-erx-fd-server you need the [Rust](https://www.rust-lang.org)
Toolchain. To setup the Toolchain follow [this instructions](https://www.rust-lang.org/learn/get-started).

## Dependencies

The ref-erx-fd-server has dependencies to external libraries listed below.

-   [OpenSSL](https://www.openssl.org) - OpenSSL is used to encrypt and decrypt requests
    send to the service. Please make sure you also have installed the development dependencies
    of OpenSSL. You will need at least openssl v1.1.1.

WARN: In openssl v1.1.1 CAdES signatures are not fully implemented. To get a valid CAdES signature
from the service you need to use openssl v3.0.0!

-   [libxml2](http://www.xmlsoft.org/) - libxml2 is used to parse and verify the XML representation of
    the TSL and the BNetzA-VL.

    -   On linux you can install the depdendencies using the following command

            $ apt-get install libssl1.1 libssl-dev libxml2-dev

    -   On windows you can download the OpenSSL binaries from [this website](https://wiki.openssl.org/index.php/Binaries)
        and the libxml2 binaries from [this website](https://www.zlatkovic.com/projects/libxml/).

Additionally you need to specify the following environment variables:

    OPENSSL_DIR = <path-to-openssl>
    OPENSSL_LIB_DIR = <path-to-openssl>/lib
    OPENSSL_INCLUDE_DIR = <path-to-openssl>/include

    LIBXML_DIR = <path-to-libxml2>
    LIBXML_LIB_DIR = <path-to-libxml2>/lib
    LIBXML_INCLUDE_DIR = <path-to-libxml2>/include

Hint: The rust compiler expects a lib-file on windows, therefore you have to rename the libxml2.dll.a
to libxml2.dll.a.lib!

## Build the Project

After you have successfully installed the Rust toolchain you can build the
project by invoking the following command.

    $ cargo build

## Generating Credentials

The service needs serveral keys to operate correctly:

-   FD keypair - key pair for the service

-   IDP keypair - key pair of the IDP service to generate access tokens
    (this is only for testing purposes, the actual access tokens are generated by the IDP service).

The cryptographic algorithms used in this example may change in the future.

To generate the needed key you can use the following Open SSL commands:

    # Generate private key of the service used for encryption
    $ openssl ecparam -name brainpoolP256r1 -genkey -noout -out fd_id_enc

    # Extract public key
    $ openssl pkey -in fd_id_enc -out fd_id_enc.pub -pubout

    # Create X509 certificate
    $ openssl req -new -key fd_id_enc > cert.csr
    $ openssl x509 -in cert.csr -out fd_id_enc.cert -req -signkey fd_id_enc -days 1001

    # Generate private key of the service used for signing
    $ openssl ecparam -name brainpoolP256r1 -genkey -noout -out fd_id_sig

    # Extract public key
    $ openssl pkey -in fd_id_sig -out fd_id_sig.pub -pubout

    # Create X509 certificate
    $ openssl req -new -key fd_id_sig > cert.csr
    $ openssl x509 -in cert.csr -out fd_id_sig.cert -req -signkey fd_id_sig -days 1001

    # Generate private key for QES signing
    $ openssl ecparam -name brainpoolP256r1 -genkey -noout -out qes_id

    # Extract public key
    $ openssl pkey -in qes_id -out qes_id.pub -pubout

    # Create X509 certificate.
    # CAUTION! Certificates created with this tool should not be used in productive environments.
    # The admission extension is not fully supported!
    $ openssl req -new -key qes_id > cert.csr
    $ cargo run -p tool -- x509 \
        --input cert.csr \
        --output qes_id.cert \
        --signkey qes_id \
        --days 1001 \
        --profession 1.2.276.0.76.4.30

    # Generate private key for the IDP service (only used for access token generation)
    $ openssl ecparam -name brainpoolP256r1 -genkey -noout -out idp_id

    # Extract public key
    $ openssl pkey -in idp_id -out idp_id.pub -pubout

    # Create X509 certificate
    $ openssl req -new -key idp_id > cert.csr
    $ openssl x509 -in cert.csr -out idp_id.cert -req -signkey idp_id -days 1001

## Run the Service

To run the service use the following command line. The service needs a
private key and X.509 certificate for the VAU encryption, a X.509 certificate
to verify the QES passed in the task activate operation, the public
key of the IDP to verify the ACCESS\_TOKEN and a download URL of the TSL.
The following parameters are mandatory.

    $ cargo run -p ref-erx-fd-server -- \
        --enc-key ./path/to/fd_id_enc \
        --enc-cert ./path/to/fd_id_enc.cert \
        --sig-key ./path/to/fd_id_sig \
        --sig-cert ./path/to/fd_id_sig.cert \
        --bnetza file://path/to/bnetzavl.xml \
        --token file://path/to/idp_id.cert \
        --tsl https://download.tsl.ti-dienste.de

For testing purposes you can use the TSL that is provided by the specified URL. In the final product you should use your own TSL endpoint!

To get a full list of all supported parameters use

    $ cargo run -p ref-erx-fd-server -- --help

You can also execute the binary of the service directly without using cargo.
The binary can be found in the target directory

    $ ./ref-erx-fd-server --help

## Create ACCESS\_TOKEN

ACCESS\_TOKEN enables the user of the service to execute different operations. The provided example generates an ACCESS\_TOKEN for a patient but patients are not allowed to create or activate Tasks (which is shown in the examples below). To successfully execute the other examples, you may need an ACCESS\_TOKEN with a different profession.

You can use the following command to generate a BP256R1 access token with the generated IDP key pair and the claims provided in [claims\_patient.json](server/examples/claims_patient.json):

    $ cargo run -p tool -- \
        create-access-token \
            --key idp_id \
            --claims server/examples/claims_patient.json

## Send Task $create Request to Server using the VAU tunnel

To send requests to the encrypted VAU tunnel of the server you can use plain text request provided in [task\_create.plain](server/examples/task_create.plain) and the following commands.

    # Create encrypted VAU payload
    $ cargo run -p tool -- \
        vau-encrypt \
            --cert fd_id_enc.cert \
            --input server/examples/task_create.plain \
            --output task_create.cipher

    # Send the encrypted request to the VAU tunnel
    curl \
        --data-binary @server/examples/task_create.cipher \
        --header "Content-Type: application/octet-stream" \
        --output response.cipher \
        http://localhost:3000/VAU/0

    # Decrypt the response
    cargo run -p tool -- \
        aes-decrypt \
            --input response.cipher \
            --key 0123456789ABCDEF0123456789ABCDEF \
            --output response.plain

## Create Encrypted QES Container for Task $activate Operation

The Task $create operation expects a signed QES container that contains the KBV bundle for the activation of the task. This example shows how to create the QES container and send it to the server.

Hint: The request would normally be send through the VAU tunnel, but for this example that is skiped. Please refere to the example above to see how VAU requests are generated.

    # Create PKCS#7 file with the KBV Bundle as Content
    $ cargo run -p tool -- \
        pkcs7-sign \
            --key qes_id \
            --cert qes_id.cert \
            --input server/examples/kbv_bundle.xml

    # Put the generated PKCS#7 file into the data field of the task activate operations payload
    $ sed -i 's/"data":".*"/"data":"MIJVIwYJK..."/g' server/examples/task_activate_parameters.json

    # Send the payload to the server
    curl \
        --data-binary @server/examples/task_activate_parameters.json \
        --header "Content-Type: application/json" \
        --header "Authorization: Bearer eyJhbG..." \
        http://localhost:3000/Task/{id}/$create

## Certificates and Trusted Service Status Lists

Some certificates that are used by the FD are validated against a so called Trusted Service Status List.
Currently we have two of these lists, passed with the '--tsl' and the '--bnetza' arguments.
The BNetz-A-VL is used to verify the QES containers used in the Task $activate operation. The TSL is used
to verify the ACCESS\_TOKEN. If you want to use your own certificates, you need to add the issuer certificate
to the corresponding list.

In order to do so, add the following lines after the last occurence of TSPService in the provided lists.

    <TSPService>
        <ServiceInformation>
            <ServiceTypeIdentifier>
                http://uri.etsi.org/TrstSvc/Svctype/CA/QC2
            </ServiceTypeIdentifier>
            <ServiceDigitalIdentity>
                <DigitalId>
                    <X509Certificate>
                        ADD_CUSTOM_CERTIFICATE_HERE
                    </X509Certificate>
                </DigitalId>
            </ServiceDigitalIdentity>
            <ServiceStatus>
                http://uri.etsi.org/TrstSvc/TrustedList/Svcstatus/granted
            </ServiceStatus>
            <StatusStartingTime>
                2020-01-01T00:00:00Z
            </StatusStartingTime>
        </ServiceInformation>
    </TSPService>

Replace "ADD\_CUSTOM\_CERTIFICATE\_HERE" with your self generated certficate. If you followed chapter
"Generating Credentials" instructions, your QES certificate could be found in qes\_id.cert. Make sure
to drop the "-----BEGIN CERTIFICATE-----" and "-----END CERTIFICATE-----" lines and remove all line
breaks. The replacement for "ADD\_CUSTOM\_CERTIFICATE\_HERE" must be a single line.

Hint: This method of adding own certifactes to the list is only valid for now. In a later release we
will check the signature of the list, so a manipulation is impossible! But we will provide a new method
to add custom certificates with the release that checks the signature.
