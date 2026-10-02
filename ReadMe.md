# cln_code_cli

A cli tool for cleaning out comments and console logs from code for more clean code.

___

## Installation

 Download the windows binary *cln_code.exe* file.

###### If you have rustup installed on your PC

1. locate the *C:/<username>/.cargo/bin*   folder and move the above mentioned binary there. 

2. Restart your terminal.

##### If you dont

1. Move to *C:* directory and create a folder called *tools* and copy the cln_code.exe into the newly created directory.

2. paste this command in your powershell

```powershell
[Environment]::SetEnvironmentVariable("Path", $env:Path + ";C:\tools", "User")
```

___

## Example

 __Before__ 

![before Image](cln_code_media/before_cln_code.jpg)



__After__



![After edit image](cln_code_media/after_cln_code.jpg)

## Usage

Run 'cln_code' in the directory containing the files you want to target

### 1. Comment out println! statements

* **target all files:** `cln_code co/cm print/log all`
* **target only one file:** `cln_code co/cm print/log [file_name]`

### 2. UnComment println! statements

* **target all files:** `cln_code uc/rc com all`
* **target only one file:** `cln_code uc/rc com [file_name]`

### 3. Delete commented line

* **target all files:** `cln_code rm com all`
* **target only one file:** `cln_code rm com [file_name]`

### 4. Delete println! or Log statement

* **target all files:** `cln_code rm print/log all`
* **target only one file:** `cln_code rm print/log [file_name]`

___

### Foreword

This project was made by @Munya-z. I learnt programming through tutorials and PDFs and I am very happy to share my code with the world. I hope you find it usefull. **Thank you.**
