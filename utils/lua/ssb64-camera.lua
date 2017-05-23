---------------
-- Libraries --
---------------
require "lib.LibScriptHawk";

-----------------------------------
-- Main SSB64 Memory Addresses   --
-----------------------------------
SSB64 = {
  Mem = { -- Versions: Japan, Australia, Europe, USA [scripthawk]
    ["screen"]              = {nil, nil, nil, 0x0A4AD0},
    ["unlockables"]         = {0x0A28F4, 0x0A5074, 0x0AD194, 0x0A4934},
    ["vs_match_global"]     = {0x0A30A8, 0x0A5828, 0x0AD948, 0x0A50E8},
    ["player_list_ptr"]     = {0x12E914, 0x131594, 0x139A74, 0x130D84},
    ["active_camera"]       = {nil, nil, nil, 0x1314B4},
    ["camera_list_ptr"]     = {nil, nil, nil, 0x12EBB4},
  }
}

---------------
-- Enums     --
---------------
chars = {
  [0x00] = "Mario",
	[0x01] = "Fox",
	[0x02] = "DK",
	[0x03] = "Samus",
	[0x04] = "Luigi",
	[0x05] = "Link",
	[0x06] = "Yoshi",
	[0x07] = "Falcon",
	[0x08] = "Kirby",
	[0x09] = "Pikachu",
	[0x0A] = "Jigglypuff",
	[0x0B] = "Ness",
	[0x0C] = "Master Hand",
	[0x0D] = "Metal Mario",
	[0x0E] = "Polygon Mario",
	[0x0F] = "Polygon Fox",
	[0x10] = "Polygon DK",
	[0x11] = "Polygon Samus",
	[0x12] = "Polygon Luigi",
	[0x13] = "Polygon Link",
	[0x14] = "Polygon Yoshi",
	[0x15] = "Polygon Falcon",
	[0x16] = "Polygon Kirby",
	[0x17] = "Polygon Pikachu",
	[0x18] = "Polygon Jigglypuff",
	[0x19] = "Polygon Ness",
	[0x1A] = "Giant DK",
	--[0x1B] = "Empty Slot",
	[0x1C] = "None", -- No character selected
}

stages = {
  vs_mode = {
    [0x00] = "Peach's Castle", -- 0x00
		[0x01] = "Sector Z",
		[0x02] = "Congo Jungle",
		[0x03] = "Planet Zebes",
		[0x04] = "Hyrule Castle",
		[0x05] = "Yoshi's Island",
		[0x06] = "Dream Land",
		[0x07] = "Saffron City",
		[0x08] = "Mushroom Kingdom",
  },
  beta = {
    [0x09] = "Dream Land Beta 1",
    [0x0A] = "Dream Land Beta 2",
    [0x0B] = "Demo Stage",
  },
  single_player = {
    [0x0C] = "Yoshi's Island no clouds",
    [0x0D] = "Metal Mario",
    [0x0E] = "Polygon Team",
    [0x0F] = "Race to the Finish!",
    [0x10] = "Final Destination", -- 0x10
    [0x11] = "Targets - Mario",
    [0x12] = "Targets - Fox",
    [0x13] = "Targets - DK",
    [0x14] = "Targets - Samus",
    [0x15] = "Targets - Luigi",
    [0x16] = "Targets - Link",
    [0x17] = "Targets - Yoshi",
    [0x18] = "Targets - Falcon",
    [0x19] = "Targets - Kirby",
    [0x1A] = "Targets - Pikachu",
    [0x1B] = "Targets - Jigglypuff",
    [0x1C] = "Targets - Ness",
    [0x1D] = "Platforms - Mario",
    [0x1E] = "Platforms - Fox",
    [0x1F] = "Platforms - DK",
    [0x20] = "Platforms - Samus", -- 0x20
    [0x21] = "Platforms - Luigi",
    [0x22] = "Platforms - Link",
    [0x23] = "Platforms - Yoshi",
    [0x24] = "Platforms - Falcon",
    [0x25] = "Platforms - Kirby",
    [0x26] = "Platforms - Pikachu",
    [0x27] = "Platforms - Jigglypuff",
    [0x28] = "Platforms - Ness", -- 0x28
  }
}

cameras = {
  [0x00] = "Battle Camera",
  [0x01] = "Character Zoom Camera",
  [0x02] = "Unknown Camera 0x2",
  [0x03] = "Unknown Camera 0x3",
  [0x04] = "BtT/BtP Pause Camera",
  [0x05] = "BtT/BtP Camera",
  [0x06] = "Unknown Camera 0x6",
}

--------------
-- Structs  --
--------------
local vs_global_settings = {
  stage = 0x01, -- u8
  match_type = 0x03, -- u8 bitflag
  match_type_flag = {
    ["time"]  = 0x01, -- 0b00000001
    ["stock"] = 0x02, -- 0b00000010
  },
  starting_time = 0x06, -- u8
  starting_stock = 0x07, -- u8
  player_base = {
    [1] = 0x20,
    [2] = 0x94,
    [3] = 0x108,
    [4] = 0x17C,
  },
  player_data = { -- Relative to player_base[player]
    controlled_by = 0x02, -- Byte: 0 Human, 1 AI, 2 None
    controlled_by_values = {
      ["MAN"] = 0x00,
      ["CPU"] = 0x01,
      ["NONE"] = 0x02,
    },
    character = 0x03, -- Byte
    damage = 0x4C, -- u32_be, Only applies to the UI, real damage is stored in the player object
  },
}

local player_fields = {
	["Character"] = 0x0B, -- Byte?
	["Costume"] = 0x10, -- Byte?
  ["PositionDataPointer"] = 0x78, -- Pointer
  ["PositionData"] = {
		["XPosition"] = 0x00, -- Float
		["YPosition"] = 0x04, -- Float
		["ZPosition"] = 0x08, -- Float
	},
}

local camera_info = {
  ['current']   = 0x00, -- u32_be
  ['previous']  = 0x04, -- u32_be
  ['camera_fn'] = 0x08, -- void (*camera)(void) [u32_be]
}

local camera_routines = {
  ["Battle Camera"]         = 0x00, -- void (*camera_fn)(void)
  ["Character Zoom Camera"] = 0x04, -- void (*camera_fn)(void)
  ["Unknown Camera 0x2"]    = 0x08, -- void (*camera_fn)(void)
  ["Unknown Camera 0x3"]    = 0x0C, -- void (*camera_fn)(void)
  ["BtT/BtP Pause Camera"]  = 0x10, -- void (*camera_fn)(void)
  ["BtT/BtP Camera"]        = 0x14, -- void (*camera_fn)(void)
  ["Unknown Camera 0x6"]    = 0x18, -- void (*camera_fn)(void)
}

---------------
-- Functions --
---------------
function getPlayerGlobal(p)
  return SSB64.Mem.vs_match_global[3] + vs_global_settings.player_base[p];
end

function getPlayerControlledBy(player)
  local globalPlayer = getPlayerGlobal(player);
  local controlled   = vs_global_settings.player_data.controlled_by;

  if isRDRAM(globalPLayer) then
    return mainmemory.readbyte(globalPlayer + controlled)
  else
    return -1 -- kinda a false...
  end
end

function setPlayerControlledBy(player, state)
  local globalPlayer = getPlayerGlobal(player);
  local controlled   = vs_global_settings.player_data.controlled_by;

  if isRDRAM(globalPLayer) then
    mainmemory.writebyte(globalPlayer + controlled, state);
  end
end

---------------
-- GUI       --
---------------
cgui = {    -- whole thing is mainly copied from [scripthawk]
	UI = {
		form_controls = {}, -- TODO: Detect UI position problems using this array
		form_padding = 8,
		label_offset = 5,
		dropdown_offset = 1,
		long_label_width = 140,
    checkbox_width = 115,
		button_height = 23,
	},
};

function cgui.setWidth(w)
  local ui = cgui.UI;
  local width = cgui.col(w);

  ui["width"] = width;

  return width
end

function cgui.row(r)
  local ui = cgui.UI;
  return round(ui.form_padding + ui.button_height * r, 0);
end

function cgui.col(c)
  local ui = cgui.UI;
  return round(ui.form_padding + ui.checkbox_width * c, 0);
end

-- Initialize Gui and save reference
cgui.UI.options_form = forms.newform(cgui.col(8), cgui.row(24), "SSB64 Camera Control");

-- Add "Hide Player" Dropdowns?
local function guiHidePlayer()
  local ui = cgui.UI.form_controls;
  local controller = cgui.UI.options_form;
  -- Add Header Label
  ui["hide_player_label"] = forms.label(controller, "---Hide Players---", cgui.col(0), cgui.row(0), cgui.col(8), cgui.row(1), false);

  for i=1, 4 do
    print(i);
    print("hide_player"..i);
    ui["hide_player"..i] = forms.checkbox(controller, "Hide Player "..i, cgui.col(i-1), cgui.row(1));
  end
  ui["test_long"] = forms.checkbox(controller, "REALLY LONG LABEL?!?!", cgui.col(0), cgui.row(2));
  ui["test_hidden"] = forms.checkbox(controller, "Can you see me?", cgui.col(1), cgui.row(2));
end

print(string.format("Player 1 Addr: %08x", getPlayerGlobal(1)));

guiHidePlayer();


--------------------
-- Event Handlers --
--------------------
