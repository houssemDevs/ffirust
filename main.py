
from ctypes import c_char_p, c_uint32, Structure, cdll, POINTER, c_size_t

class c_mentry(Structure):
    _fields_ = [("file", c_char_p),
                ("kind", c_char_p),
                ("name", c_char_p),
                ("row", c_uint32),
                ("col", c_uint32),]
    def __str__(self):
        return "({},{},{},({},{}))".format(self.file,self.kind,self.name,self.row,self.col)

class c_mentries(Structure):
    _fields_ = [("entries", POINTER(c_mentry)),
                ("len", c_size_t)]
    def __str__(self):
        return "{} items, {}".format(self.len, self.entries)

lib = cdll.LoadLibrary("target/debug/dbacp.dll")

lib.get_entry.restype = c_mentry
lib.get_entries.restype = c_mentries

print(lib.get_entry())
entries = lib.get_entries()
for i in range(0,entries.len):
    print(entries.entries[i].name, entries.entries[i].kind, entries.entries[i].col, entries.entries[i].row)
