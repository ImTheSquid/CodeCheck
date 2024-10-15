import google.generativeai as genai
import os
from pathlib import Path
import time

lang_from_name = "java"
lang_from_ext = "java"
lang_to_name = "python"
lang_to_ext = "py"

inst = f"output only {lang_to_name} code that does the same thing in the same method as the code file uploaded. the generated python code must run properly. Some exaples you will see require an input to be passed. In these cases, small modifications to the python code may be made so that nothing needs to passed to STDIN, but you may only do this while testing. The final returned python code must be the best match to the supplied code as is possible."
# validate_inst = f"read the {lang_to_name} code and run it to ensure it works properly. If it does not, please return the word 'FAILURE' and nothing else. If it works, return the word 'SUCCESS' and nothing else." 


genai.configure(api_key=os.environ["GEMINI_API_KEY"])

gen_model = genai.GenerativeModel(model_name="gemini-1.5-flash", system_instruction=inst, tools='code_execution')

# val_model = genai.GenerativeModel(model_name="gemini-1.5-flash", system_instruction=validate_inst, tools='code_execution')

def regen_paths():
    rel_file_paths = []

    for root, dirs, files in os.walk(os.path.join(os.getcwd(), "input")):
            for file in files:
                # print(file)
                rel_file_path = os.path.relpath(os.path.join(root, file), os.getcwd())
                # print(rel_file_path)
                rel_file_paths.append(rel_file_path)  

    # print(rel_file_paths)
    # exit()
    ct = 0
    for rel_file_path in rel_file_paths:
        ct += 1
        print(f"\n-----------------------------------FILE {ct}-----------------------------------")
        print(f"Attempting to convert {rel_file_path}")
        if rel_file_path.endswith(lang_from_ext):

            liminal_file_path = "/".join(rel_file_path.split("/")[1:])
            # print(f"Converting {liminal_file_path}")


            try:
                response = convert(liminal_file_path)
            except Exception as e:
                print("\n---Quota exhausted, waiting 1 min---")
                print(f"Error: {e}")
                time.sleep(60)

            # print(liminal_file_path)
            output_file = Path(f"{os.getcwd()}/output/{liminal_file_path}")
            output_file.parent.mkdir(exist_ok=True, parents=True)

            output_file_path = liminal_file_path.split(".")[0] + f".{lang_to_ext}"

            with open(f"output/{output_file_path}", "w") as file:
                file.write("\n".join(response.split("\n")[1:-1]))
                print(f"Saved output to output/{output_file_path}")
        else:
            print(f"Not a .{lang_from_ext} file, skipping")


def convert(i):
    # inp = os.listdir("input")
    # for i in inp:
    #     if i.endswith(lang_from_ext):
    # filename = i[:-len(lang_from_ext)-1]
    # print(f"Called convert with {i}")
    filename = i.split("/")[-1][:-len(lang_from_ext)-1]

    file_upload = genai.upload_file(path=f"input/{i}", display_name=f"input file: {i}", mime_type="text/plain")
    print(f"Uploaded file '{file_upload.display_name}' as: {file_upload.uri}")

    response = gen_model.generate_content(file_upload).text
    print(f"Generated translation: '{filename}.{lang_from_ext}' -> '{filename}.{lang_to_ext}'")

    return response

    # with open(f"output/{filename}.{lang_to_ext}", "w") as file:
        # file.write("\n".join(response.split("\n")[1:-1]))

    # print(f"translation response: {response}")

            # if not validate(filename):
            #     print(f"Validation failed for {filename}.{lang_to_ext}")
            #     os.remove(f"output/{filename}.{lang_to_ext}")


# def validate(filename):
    # with open(f"output/{filename}.{lang_to_ext}", "r") as file:
    #     code = file.read()

    # response = val_model.generate_content(code).text
    # print(f"Validation response: {response}")
    # if response.count("SUCCESS"):
    #     return True
    # elif response.count("FAILURE"):
    #     return False
    # else:
    #     print("Validation failed, assuming failure")
    #     return False


if __name__ == "__main__":
    # convert()
    regen_paths()