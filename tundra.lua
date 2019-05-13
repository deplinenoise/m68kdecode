
local common = {
  Env = {
    -- Global, shared environment settings can go here
    CPPPATH = { ".", "$(OBJECTDIR)", "src" },
    VASM = "../newage_a500/t2-output/hostbuild/vasm",
  },
  Defines = {
    { "_DEBUG"; Config = '*-*-debug' },
    { "NDEBUG"; Config = '*-*-release' },
  },
}

local win64 = {
  Inherit = common,
  Env = {
    GENERATE_PDB = "1",
    CCOPTS = {
      "/FS",
      "/W4",
      { "/Od"; Config = "*-*-debug" },
      { "/O2"; Config = "*-*-production" },
      { "/Ox"; Config = "*-*-release" },
    },
  },
}

local macosx = {
  Inherit = common,
  Env = {
    CCOPTS = {
      { "-O0", "-g", "-fsanitize=address"; Config = "*-*-debug" },
      { "-O2", "-g"; Config = "*-*-production" },
      { "-O3"; Config = "*-*-release" },
    },
    PROGOPTS = {
      { "-fsanitize=address"; Config = "*-*-debug" },
    }
  },
}

local linux = {
  Inherit = common,
  Env = {
    CCOPTS = {
      { "-O0", "-g"; Config = "*-*-debug" },
      { "-O2", "-g"; Config = "*-*-production" },
      { "-O3"; Config = "*-*-release" },
    },
  },
}

Build {
  Passes = {
    -- Define any additional passes you need here for reliable code generation/include interactions.
    CodeGen = { Name = "Code Generation", BuildOrder = 1 },
  },
  Configs = {
    Config {
      Name = "win64-mscv",
      SupportedHosts = { "windows" },
      DefaultOnHost = "windows",
      Tools = { "msvc" },
      Inherit = win64,
    },
    Config {
      Name = "macosx-clang",
      SupportedHosts = { "macosx" },
      DefaultOnHost = "macosx",
      Tools = { "clang-osx" },
      Inherit = macosx,
    },
    Config {
      Name = "linux-gcc",
      SupportedHosts = { "linux" },
      DefaultOnHost = "linux",
      Tools = { "gcc" },
      Inherit = linux,
    },
  },

  Units = function()
    DefRule {
      Name = "GenDecoder",
      Pass = "CodeGen",
      Blueprint = {},
      Command = "python gendecoder.py $(<) $(@:[1]) $(@:[2])",
      ImplicitInputs = {
        "gendecoder.py",
      },
      Setup = function (env, data)
        return {
          InputFiles = { "imask.txt" },
          OutputFiles = { "$(OBJECTDIR)/decoder.c", "$(OBJECTDIR)/decoder.h" },
        }
      end,
    }

    DefRule {
      Name = "GenTests",
      Pass = "CodeGen",
      Blueprint = {},
      Command = "python gen_decoding_tests.py $(VASM) $(<) $(@)",
      ImplicitInputs = {
        "gen_decoding_tests.py",
      },
      Setup = function (env, data)
        return {
          InputFiles = { "tests/decode_tests.txt" },
          OutputFiles = { "$(OBJECTDIR)/decode_tests.c" },
        }
      end,
    }

    local lib = StaticLibrary {
      Name = "m68kdecode",
      Sources = {
        GenDecoder {},
        "src/lib.h",
        "src/lib.c",
        "src/decoder.h",
        "src/decoder_support.inl",
      },
    }
    local dectest = Program {
      Name = "dectest",
      Depends = { lib },
      Sources = {
        GenTests {},
        "tests/test_support.inl",
      }
    }

    Default(lib)
    Default(dectest)
  end,
}

