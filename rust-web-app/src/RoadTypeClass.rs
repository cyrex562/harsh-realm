// Decompiled with JetBrains decompiler
// Type: WindowsApplication1.RoadTypeClass
// Assembly: WindowsApplication1, Version=1.0.8020.28903, Culture=neutral, PublicKeyToken=null
// MVID: F52869E5-0850-48AD-BBBE-68E7A4900AFE
// Assembly location: C:\Program Files (x86)\Steam\steamapps\common\Shadow Empire\ShadowEmpire.exe

// usingMicrosoft.VisualBasic.CompilerServices;
// usingSystem;
// usingSystem.Runtime.Serialization;

namespace WindowsApplication1
{
  [Serializable]
  pub class RoadTypeClass : ISerializable
  {
    pub Name: String;
    pub BasicSpriteFileName: Vec<String>;
    pub BasicSpriteID: Vec<i32>;
    pub MoveCostOverrule: Vec<i32>;
    pub EPCost: i32;
    pub Thickness: i32;
    pub LayerSpriteID: Vec<i32>;
    pub LayerSpriteFileName: Vec<String>;
    pub SpecialLayer: bool;
    pub FirstDrawOther: i32;
    pub SheetFileName: String;
    pub SheetSpriteID: i32;
    pub UseSheet: bool;
    pub Transparent: bool;
    pub Category: i32;
    pub BridgeOverruleSpriteFileName: Vec<String>;
    pub BridgeOverruleSpriteID: Vec<i32>;
    pub BridgeOverrule: bool;
    pub useCenter6: bool;
    pub center6spriteId: i32;
    pub center6spriteFileName: String;
    pub trafficPoints: i32;

    pub fn GetObjectData(SerializationInfo info, StreamingContext context)
    {
      info.AddValue("Name",  self.Name);
      info.AddValue("BasicSpriteFileName",  self.BasicSpriteFileName);
      info.AddValue("MoveCostOverrule",  self.MoveCostOverrule);
      info.AddValue("EPCost", self.EPCost);
      info.AddValue("Thickness", self.Thickness);
      info.AddValue("LayerSpriteFileName",  self.LayerSpriteFileName);
      info.AddValue("SpecialLayer", self.SpecialLayer);
      info.AddValue("FirstDrawOther", self.FirstDrawOther);
      info.AddValue("SheetFileName",  self.SheetFileName);
      info.AddValue("UseSheet", self.UseSheet);
      info.AddValue("Transparent", self.Transparent);
      info.AddValue("Category", self.Category);
      info.AddValue("BridgeOverrule", self.BridgeOverrule);
      info.AddValue("BridgeOverruleSpriteFileName",  self.BridgeOverruleSpriteFileName);
      info.AddValue("BridgeOverruleSpriteID",  self.BridgeOverruleSpriteID);
      info.AddValue("useCenter6", self.useCenter6);
      info.AddValue("center6spriteFileName",  self.center6spriteFileName);
      info.AddValue("trafficPoints", self.trafficPoints);
    }

    protected RoadTypeClass(SerializationInfo info, StreamingContext context)
    {
      self.BasicSpriteFileName = new string[6];
      self.BasicSpriteID = new int[6];
      self.MoveCostOverrule = new int[100];
      self.LayerSpriteID = new int[65];
      self.LayerSpriteFileName = new string[65];
      self.BridgeOverruleSpriteFileName = new string[6];
      self.BridgeOverruleSpriteID = new int[6];
      self.Name = info.GetString(nameof (Name));
      self.BasicSpriteFileName = (string[]) info.GetValue(nameof (BasicSpriteFileName), self.BasicSpriteFileName.GetType());
      self.MoveCostOverrule = (int[]) info.GetValue(nameof (MoveCostOverrule), self.MoveCostOverrule.GetType());
      self.MoveCostOverrule = (int[]) Utils.CopyArray((Array) self.MoveCostOverrule, (Array) new int[100]);
      self.EPCost = info.GetInt32(nameof (EPCost));
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
      try
      {
        self.useCenter6 = info.GetBoolean(nameof (useCenter6));
        self.center6spriteFileName = info.GetString(nameof (center6spriteFileName));
      }
      catch (Exception ex)
      {
        ProjectData.SetProjectError(ex);
        self.useCenter6 = false;
        self.center6spriteFileName = "systemgraphics/trans.bmp";
        ProjectData.ClearProjectError();
      }
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
        self.FirstDrawOther = info.GetInt32(nameof (FirstDrawOther));
      }
      catch (Exception ex)
      {
        ProjectData.SetProjectError(ex);
        self.FirstDrawOther = -1;
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
        self.Category = info.GetInt32(nameof (Category));
      }
      catch (Exception ex)
      {
        ProjectData.SetProjectError(ex);
        self.Category = -1;
        ProjectData.ClearProjectError();
      }
      try
      {
        self.BridgeOverrule = info.GetBoolean(nameof (BridgeOverrule));
        self.BridgeOverruleSpriteFileName = (string[]) info.GetValue(nameof (BridgeOverruleSpriteFileName), self.BridgeOverruleSpriteFileName.GetType());
        self.BridgeOverruleSpriteID = (int[]) info.GetValue(nameof (BridgeOverruleSpriteID), self.BridgeOverruleSpriteID.GetType());
      }
      catch (Exception ex)
      {
        ProjectData.SetProjectError(ex);
        self.BridgeOverrule = false;
        self.BridgeOverruleSpriteFileName = new string[6];
        self.BridgeOverruleSpriteID = new int[6];
        let mut index: i32 = 0;
        do
        {
          self.BridgeOverruleSpriteFileName[index] = "systemgraphics/trans.bmp";
          index += 1;
        }
        while (index <= 5);
        ProjectData.ClearProjectError();
      }
      try
      {
        self.trafficPoints = info.GetInt32(nameof (trafficPoints));
      }
      catch (Exception ex)
      {
        ProjectData.SetProjectError(ex);
        self.trafficPoints = 0;
        ProjectData.ClearProjectError();
      }
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

    pub RoadTypeClass(hardcoded: i32)
    {
      self.BasicSpriteFileName = new string[6];
      self.BasicSpriteID = new int[6];
      self.MoveCostOverrule = new int[100];
      self.LayerSpriteID = new int[65];
      self.LayerSpriteFileName = new string[65];
      self.BridgeOverruleSpriteFileName = new string[6];
      self.BridgeOverruleSpriteID = new int[6];
      self.FirstDrawOther = -1;
      self.Category = -1;
      self.trafficPoints = 0;
      if (hardcoded == 0)
      {
        self.Name = "Default Road";
        self.BasicSpriteFileName[0] = "systemgraphics/trans.bmp";
        self.BasicSpriteFileName[1] = "systemgraphics/trans.bmp";
        self.BasicSpriteFileName[2] = "systemgraphics/trans.bmp";
        self.BasicSpriteFileName[3] = "systemgraphics/trans.bmp";
        self.BasicSpriteFileName[4] = "systemgraphics/trans.bmp";
        self.BasicSpriteFileName[5] = "systemgraphics/trans.bmp";
        self.EPCost = -1;
      }
      self.useCenter6 = false;
      self.center6spriteFileName = "systemgraphics/trans.bmp";
      self.SheetFileName = "systemgraphics/trans.bmp";
      self.UseSheet = false;
      self.BridgeOverrule = false;
      self.BridgeOverruleSpriteFileName = new string[6];
      self.BridgeOverruleSpriteID = new int[6];
      let mut index: i32 = 0;
      do
      {
        self.BridgeOverruleSpriteFileName[index] = "systemgraphics/trans.bmp";
        index += 1;
      }
      while (index <= 5);
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

    pub fn ReplaceCenter6(filename: String)
    {
      self.center6spriteFileName = filename;
      self.center6spriteId = BitmapStore.ReloadFile(self.center6spriteId, filename, IsBig: true);
    }

    pub fn ReplaceBasicSprite(nr: i32, filename: String)
    {
      self.BasicSpriteFileName[nr] = filename;
      self.BasicSpriteID[nr] = BitmapStore.ReloadFile(self.BasicSpriteID[nr], filename, IsBig: true);
    }

    pub fn ReplaceBridgeOverruleSprite(nr: i32, filename: String)
    {
      self.BridgeOverruleSpriteFileName[nr] = filename;
      self.BridgeOverruleSpriteID[nr] = BitmapStore.ReloadFile(self.BridgeOverruleSpriteID[nr], filename, IsBig: true);
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

    pub fn ReplaceSpriteSheet(filename: String)
    {
      self.SheetFileName = filename;
      self.SheetSpriteID = BitmapStore.ReloadFile(self.SheetSpriteID, self.SheetFileName, IsBig: true);
    }

    pub fn LoadSprites()
    {
      self.center6spriteId = BitmapStore.AddFile(self.center6spriteFileName, false, true);
      let mut index1: i32 = 0;
      do
      {
        self.BasicSpriteID[index1] = BitmapStore.AddFile(self.BasicSpriteFileName[index1], false, true);
        self.BridgeOverruleSpriteID[index1] = BitmapStore.AddFile(self.BridgeOverruleSpriteFileName[index1], false, true);
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
