# rhh

Calculating and Verifying Hash Value

## Supported algorithms

* MD5
* SHA1
* SHA256

## Usage

### Calculate a hash value from stdin

```sh
./rhh
```

* Please send an EOF to decide your input.
  * Windows (e.g., Command Prompt, PowerShell, nushell, ...): Press Ctrl + Z to send EOF.
  * macOS, Linux, Unix (e.g., sh, csh, bash, zsh, fish, ...): Press Ctrl + D to send EOF.

### Calculate a hash value from file

```sh
./rhh FILE1 FILE2 ...
```

* You can specify one or more files.

### Verify the file

```sh
./rhh -c CHECKSUM [--ignore | --ignore-missing]
```

* Specify only one CHECKSUM.
* If `OK` is displayed, the verification is successful. `FAILED` indicates a hash value mismatch.
* If the --ignore or --ignore-missing option is specified, the missing files listed in CHECKSUM are ignored.

### Initialize Hash DB

```sh
./rhh --init-db
```

* The hash value of the found by recursively searching the current directory are stored in in the hash DB.
* If the hash DB already exists, remove the DB file, and regenerate it.

### Update Hash DB

```sh
./rhh --update-db
```

* The hash value of the file found by recursively searching the current directory are stored in the hash DB.
* If the hash DB does not exist, the command fails.
* Files that do not exist in the hash DB are stored in the hash DB.
* Files that in the hash DB and on the filesystem are stored in the DB by recalculating the hash values.
* Files that exist in the hash DB but are not on the filesystem are ignored.

### Search Hash DB

```sh
./rhh --use-db FILE1 FILE2 ...
```

* You can specify one or more files.

### Others

#### Version
  
```sh
./rhh --version
```

#### Help

```sh
./rhh --help
```
