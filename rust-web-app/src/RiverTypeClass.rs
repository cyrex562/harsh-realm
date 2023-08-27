// Decompiled with JetBrains decompiler
// Type: WindowsApplication1.RiverTypeClass
// Assembly: WindowsApplication1, Version=1.0.8020.28903, Culture=neutral, PublicKeyToken=null
// MVID: F52869E5-0850-48AD-BBBE-68E7A4900AFE
// Assembly location: C:\Program Files (x86)\Steam\steamapps\common\Shadow Empire\ShadowEmpire.exe

// usingMicrosoft.VisualBasic;
// usingMicrosoft.VisualBasic.CompilerServices;
// usingSystem;
// usingSystem.Runtime.Serialization;

namespace WindowsApplication1
{
  [Serializable]
  pub class RiverTypeClass : ISerializable
  {
    pub Name: String;
    pub BasicSpriteFileName: Vec<String>;
    pub BasicSpriteID: Vec<i32>;
    pub LayerSpriteID: Vec<i32>;
    pub LayerSpriteFileName: Vec<String>;
    pub SpecialLayer: bool;
    pub float[] AttackPenalty;
    pub MovePenalty: Vec<i32>;
    pub BridgePossible: bool;
    pub float BridgeCostModifier;
    pub SheetFileName: String;
    pub SheetSpriteID: i32;
    pub UseSheet: bool;
    pub Transparent: bool;
    pub TempDefenseBonus: i32;
    pub snakeMode: bool;
    pub Thickness: i32;
    pub drawInteriorOnly: bool;

    pub fn GetObjectData(SerializationInfo info, StreamingContext context)
    {
      info.AddValue("Name",  self.Name);
      info.AddValue("BasicSpriteFileName",  self.BasicSpriteFileName);
      info.AddValue("AttackPenalty",  self.AttackPenalty);
      info.AddValue("MovePenalty",  self.MovePenalty);
      info.AddValue("BridgePossible", self.BridgePossible);
      info.AddValue("BridgeCostModifier", self.BridgeCostModifier);
      info.AddValue("LayerSpriteFileName",  self.LayerSpriteFileName);
      info.AddValue("SpecialLayer", self.SpecialLayer);
      info.AddValue("SheetFileName",  self.SheetFileName);
      info.AddValue("UseSheet", self.UseSheet);
      info.AddValue("Transparent", self.Transparent);
      info.AddValue("snakeMode", self.snakeMode);
      info.AddValue("drawInteriorOnly", self.drawInteriorOnly);
      info.AddValue("Thickness", self.Thickness);
    }

    protected RiverTypeClass(SerializationInfo info, StreamingContext context)
    {
      self.BasicSpriteFileName = new string[6];
      self.BasicSpriteID = new int[6];
      self.LayerSpriteID = new int[65];
      self.LayerSpriteFileName = new string[65];
      self.AttackPenalty = new float[100];
      self.MovePenalty = new int[100];
      self.Name = info.GetString(nameof (Name));
      self.BasicSpriteFileName = (string[]) info.GetValue(nameof (BasicSpriteFileName), self.BasicSpriteFileName.GetType());
      self.AttackPenalty = (float[]) info.GetValue(nameof (AttackPenalty), self.AttackPenalty.GetType());
      self.AttackPenalty = (float[]) Utils.CopyArray((Array) self.AttackPenalty, (Array) new float[100]);
      self.MovePenalty = (int[]) info.GetValue(nameof (MovePenalty), self.MovePenalty.GetType());
      self.MovePenalty = (int[]) Utils.CopyArray((Array) self.MovePenalty, (Array) new int[100]);
      self.BridgePossible = info.GetBoolean(nameof (BridgePossible));
      self.BridgeCostModifier = info.GetSingle(nameof (BridgeCostModifier));
      try
      {
        self.LayerSpriteFileName = (string[]) info.GetValue(nameof (LayerSpriteFileName), self.LayerSpriteFileName.GetType());
        self.SpecialLayer = info.GetBoolean(nameof (SpecialLayer));
      }
      catch (Exception ex)
      {
        ProjectData.SetProjectError(ex);
        self.LayerSpriteFileName = new string[65];
        self.SpecialLayer = false;
        ProjectData.ClearProjectError();
      }
      try
      {
        self.SheetFileName = info.GetString(nameof (SheetFileName));
        self.UseSheet = info.GetBoolean(nameof (UseSheet));
      }
      catch (Exception ex)
      {
        ProjectData.SetProjectError(ex);
        self.SheetFileName = "systemgraphics/trans.bmp";
        self.UseSheet = false;
        ProjectData.ClearProjectError();
      }
      try
      {
        self.Transparent = info.GetBoolean(nameof (Transparent));
      }
      catch (Exception ex)
      {
        ProjectData.SetProjectError(ex);
        self.Transparent = false;
        ProjectData.ClearProjectError();
      }
      try
      {
        self.snakeMode = info.GetBoolean(nameof (snakeMode));
      }
      catch (Exception ex)
      {
        ProjectData.SetProjectError(ex);
        self.snakeMode = false;
        ProjectData.ClearProjectError();
      }
      try
      {
        self.drawInteriorOnly = info.GetBoolean(nameof (drawInteriorOnly));
      }
      catch (Exception ex)
      {
        ProjectData.SetProjectError(ex);
        self.drawInteriorOnly = false;
        ProjectData.ClearProjectError();
      }
      try
      {
        self.Thickness = info.GetInt32(nameof (Thickness));
      }
      catch (Exception ex)
      {
        ProjectData.SetProjectError(ex);
        self.Thickness = 1;
        ProjectData.ClearProjectError();
      }
    }

    pub fn GetRiverHeight(game: GameClass, x: i32, y: i32, z: i32) -> i32
    {
      if (!game.AllowHeightMap ||  game.Data.RuleVar[418] < 1.0)
        return 0;
      let mut num1: i32 = game.Data.MapObj[0].HexObj[x, y].RiverType[z];
      if (num1 < 0)
        return 0;
      Coordinate coordinate = game.HandyFunctionsObj.HexNeighbour(x, y, 0, z + 1);
      if (game.Data.LandscapeTypeObj[game.Data.MapObj[0].HexObj[x, y].LandscapeType].IsSea)
        return 0;
      let mut num2: i32 = 0;
      if (coordinate.onmap)
      {
        if (!game.Data.LandscapeTypeObj[game.Data.MapObj[0].HexObj[x, y].LandscapeType].IsSea)
          num2 = Math.Min(game.Data.MapObj[0].HexObj[x, y].HeightLevel, game.Data.MapObj[0].HexObj[coordinate.x, coordinate.y].HeightLevel);
      }
      else
        num2 = game.Data.MapObj[0].HexObj[x, y].HeightLevel;
      if (num2 <= 0)
        return 0;
      let mut stringListById: i32 = game.HandyFunctionsObj.GetStringListByID( Math.Round( game.Data.RuleVar[418]));
      bool flag = false;
      if (stringListById > -1)
      {
        let mut length: i32 = game.Data.StringListObj[stringListById].Length;
        for (let mut index: i32 = 0; index <= length; index += 1)
        {
          if ( Math.Round(Conversion.Val(game.Data.StringListObj[stringListById].Data[index, 0])) == num1)
          {
            if ( Math.Round(Conversion.Val(game.Data.StringListObj[stringListById].Data[index, 1])) <= num2 &&  Math.Round(Conversion.Val(game.Data.StringListObj[stringListById].Data[index, 2])) >= num2)
              return 1;
            flag = true;
          }
        }
      }
      return 0;
    }

    pub RiverTypeClass(hardcoded: i32)
    {
      self.BasicSpriteFileName = new string[6];
      self.BasicSpriteID = new int[6];
      self.LayerSpriteID = new int[65];
      self.LayerSpriteFileName = new string[65];
      self.AttackPenalty = new float[100];
      self.MovePenalty = new int[100];
      if (hardcoded == 0)
      {
        self.Name = "Default River";
        self.BasicSpriteFileName[0] = "systemgraphics/trans.bmp";
        self.BasicSpriteFileName[1] = "systemgraphics/trans.bmp";
        self.BasicSpriteFileName[2] = "systemgraphics/trans.bmp";
        self.BasicSpriteFileName[3] = "systemgraphics/trans.bmp";
        self.BasicSpriteFileName[4] = "systemgraphics/trans.bmp";
        self.BasicSpriteFileName[5] = "systemgraphics/trans.bmp";
      }
      self.SheetFileName = "systemgraphics/trans.bmp";
      self.UseSheet = false;
      self.drawInteriorOnly = false;
      self.snakeMode = false;
    }

    pub fn AutoLoadSpecial(dirstring: String, extstring: String)
    {
      self.LayerSpriteFileName[1] = dirstring + "/a1" + extstring;
      self.LayerSpriteFileName[2] = dirstring + "/b1" + extstring;
      self.LayerSpriteFileName[3] = dirstring + "/b2" + extstring;
      self.LayerSpriteFileName[4] = dirstring + "/b3" + extstring;
      self.LayerSpriteFileName[5] = dirstring + "/b4" + extstring;
      self.LayerSpriteFileName[6] = dirstring + "/b5" + extstring;
      self.LayerSpriteFileName[7] = dirstring + "/b6" + extstring;
      self.LayerSpriteFileName[8] = dirstring + "/c1" + extstring;
      self.LayerSpriteFileName[9] = dirstring + "/c2" + extstring;
      self.LayerSpriteFileName[10] = dirstring + "/c3" + extstring;
      self.LayerSpriteFileName[11] = dirstring + "/c4" + extstring;
      self.LayerSpriteFileName[12] = dirstring + "/c5" + extstring;
      self.LayerSpriteFileName[13] = dirstring + "/c6" + extstring;
      self.LayerSpriteFileName[14] = dirstring + "/c7" + extstring;
      self.LayerSpriteFileName[15] = dirstring + "/c8" + extstring;
      self.LayerSpriteFileName[16] = dirstring + "/c9" + extstring;
      self.LayerSpriteFileName[17] = dirstring + "/c10" + extstring;
      self.LayerSpriteFileName[18] = dirstring + "/c11" + extstring;
      self.LayerSpriteFileName[19] = dirstring + "/c12" + extstring;
      self.LayerSpriteFileName[20] = dirstring + "/c13" + extstring;
      self.LayerSpriteFileName[21] = dirstring + "/c14" + extstring;
      self.LayerSpriteFileName[22] = dirstring + "/c15" + extstring;
      self.LayerSpriteFileName[23] = dirstring + "/d1" + extstring;
      self.LayerSpriteFileName[24] = dirstring + "/d2" + extstring;
      self.LayerSpriteFileName[25] = dirstring + "/d3" + extstring;
      self.LayerSpriteFileName[26] = dirstring + "/d4" + extstring;
      self.LayerSpriteFileName[27] = dirstring + "/d5" + extstring;
      self.LayerSpriteFileName[28] = dirstring + "/d6" + extstring;
      self.LayerSpriteFileName[29] = dirstring + "/d7" + extstring;
      self.LayerSpriteFileName[30] = dirstring + "/d8" + extstring;
      self.LayerSpriteFileName[31] = dirstring + "/d9" + extstring;
      self.LayerSpriteFileName[32] = dirstring + "/d10" + extstring;
      self.LayerSpriteFileName[33] = dirstring + "/d11" + extstring;
      self.LayerSpriteFileName[34] = dirstring + "/d12" + extstring;
      self.LayerSpriteFileName[35] = dirstring + "/d13" + extstring;
      self.LayerSpriteFileName[36] = dirstring + "/d14" + extstring;
      self.LayerSpriteFileName[37] = dirstring + "/d15" + extstring;
      self.LayerSpriteFileName[38] = dirstring + "/d16" + extstring;
      self.LayerSpriteFileName[39] = dirstring + "/d17" + extstring;
      self.LayerSpriteFileName[40] = dirstring + "/d18" + extstring;
      self.LayerSpriteFileName[41] = dirstring + "/d19" + extstring;
      self.LayerSpriteFileName[42] = dirstring + "/d20" + extstring;
      self.LayerSpriteFileName[43] = dirstring + "/e1" + extstring;
      self.LayerSpriteFileName[44] = dirstring + "/e2" + extstring;
      self.LayerSpriteFileName[45] = dirstring + "/e3" + extstring;
      self.LayerSpriteFileName[46] = dirstring + "/e4" + extstring;
      self.LayerSpriteFileName[47] = dirstring + "/e5" + extstring;
      self.LayerSpriteFileName[48] = dirstring + "/e6" + extstring;
      self.LayerSpriteFileName[49] = dirstring + "/e7" + extstring;
      self.LayerSpriteFileName[50] = dirstring + "/e8" + extstring;
      self.LayerSpriteFileName[51] = dirstring + "/e9" + extstring;
      self.LayerSpriteFileName[52] = dirstring + "/e10" + extstring;
      self.LayerSpriteFileName[53] = dirstring + "/e11" + extstring;
      self.LayerSpriteFileName[54] = dirstring + "/e12" + extstring;
      self.LayerSpriteFileName[55] = dirstring + "/e13" + extstring;
      self.LayerSpriteFileName[56] = dirstring + "/e14" + extstring;
      self.LayerSpriteFileName[57] = dirstring + "/e15" + extstring;
      self.LayerSpriteFileName[58] = dirstring + "/f1" + extstring;
      self.LayerSpriteFileName[59] = dirstring + "/f2" + extstring;
      self.LayerSpriteFileName[60] = dirstring + "/f3" + extstring;
      self.LayerSpriteFileName[61] = dirstring + "/f4" + extstring;
      self.LayerSpriteFileName[62] = dirstring + "/f5" + extstring;
      self.LayerSpriteFileName[63] = dirstring + "/f6" + extstring;
      self.LayerSpriteFileName[64] = dirstring + "/g1" + extstring;
      self.ReloadSpecialSprites();
    }

    pub fn Kill()
    {
      let mut index1: i32 = 0;
      do
      {
        BitmapStore.RemoveBitmapNr(self.BasicSpriteID[index1]);
        index1 += 1;
      }
      while (index1 <= 5);
      if (!self.SpecialLayer || self.UseSheet)
        return;
      let mut index2: i32 = 1;
      do
      {
        BitmapStore.RemoveBitmapNr(self.LayerSpriteID[index2]);
        index2 += 1;
      }
      while (index2 <= 64);
    }

    pub fn ReplaceBasicSprite(nr: i32, filename: String)
    {
      self.BasicSpriteFileName[nr] = filename;
      self.BasicSpriteID[nr] = BitmapStore.ReloadFile(self.BasicSpriteID[nr], filename, IsBig: true);
    }

    pub fn ReplaceSpriteSheet(filename: String)
    {
      self.SheetFileName = filename;
      self.SheetSpriteID = BitmapStore.ReloadFile(self.SheetSpriteID, self.SheetFileName, IsBig: true);
    }

    pub fn ReloadSpecialSprites()
    {
      let mut index: i32 = 1;
      do
      {
        self.LayerSpriteID[index] = self.LayerSpriteID[index] <= 0 ? BitmapStore.AddFile(self.LayerSpriteFileName[index], false, true) : BitmapStore.ReloadFile(self.LayerSpriteID[index], self.LayerSpriteFileName[index], IsBig: true);
        index += 1;
      }
      while (index <= 64);
    }

    pub fn LoadSprites()
    {
      let mut index1: i32 = 0;
      do
      {
        self.BasicSpriteID[index1] = BitmapStore.AddFile(self.BasicSpriteFileName[index1], false, true);
        index1 += 1;
      }
      while (index1 <= 5);
      if (self.SpecialLayer && !self.UseSheet)
      {
        let mut index2: i32 = 1;
        do
        {
          self.LayerSpriteID[index2] = BitmapStore.AddFile(self.LayerSpriteFileName[index2], false, true);
          index2 += 1;
        }
        while (index2 <= 64);
      }
      self.SheetSpriteID = BitmapStore.AddFile(self.SheetFileName, false, true);
    }
  }
}
