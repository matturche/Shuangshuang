import unicodedata


def clean_file_for_shuangshuang(in_path_1: str, in_path_2: str, out_path: str):
    if out_path.endswith(".txt"):
        with open(out_path, "w", encoding="utf-8") as out_f:
            missing_record_set = set()
            with open(in_path_2, "r", encoding="utf-8") as in_f_2:
                for line in in_f_2:
                    if character_is_hanzi(line[0]):
                        missing_record_set.add(line)
            if in_path_1.endswith(".txt"):
                with open(in_path_1, "r", encoding="utf-8") as in_f_1:
                    for line in in_f_1:
                        if (
                            character_is_hanzi(line[0])
                            # and len(line) == 3
                            and count_cjk_chars(line) == 2
                            and line not in missing_record_set
                        ):
                            out_f.write(line)


def character_is_hanzi(character: str) -> bool:
    return "\u4E00" <= character[0] <= "\u9FFF"


def text_contains_hanzi(text: str) -> bool:
    for character in text:
        if character_is_hanzi(character):
            return True
    return False


def count_cjk_chars(text: str) -> int:
    """Count numbers of CJK characters in a string.

    Arg:
        text (str): The string contains CJK characters.

    Returns:
        int: The number of CJK characters.
    """
    if not (type(text) is str):
        raise TypeError("count_cjk_str only accept string.")
    counts = 0
    for c in text:
        if unicodedata.east_asian_width(c) in 'WF':
            counts += 1
    return counts


if __name__ == "__main__":
    clean_file_for_shuangshuang(
        "../data/HSK2012_all.txt",
        "../data/in_HSK2012_all_missing-audios.o.txt",
        "../data/output.txt",
    )
    print("Done!")
