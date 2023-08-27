// Decompiled with JetBrains decompiler
// Type: WindowsApplication1.LandscapeTypeClass
// Assembly: WindowsApplication1, Version=1.0.8020.28903, Culture=neutral, PublicKeyToken=null
// MVID: F52869E5-0850-48AD-BBBE-68E7A4900AFE
// Assembly location: C:\Program Files (x86)\Steam\steamapps\common\Shadow Empire\ShadowEmpire.exe

// usingMicrosoft.VisualBasic.CompilerServices;
// usingSystem;
// usingSystem.Drawing;
// usingSystem.IO;
// usingSystem.Runtime.Serialization;
// usingSystem.Runtime.Serialization.Formatters.Binary;

namespace WindowsApplication1
{
  [Serializable]
  pub class LandscapeTypeClass : ISerializable
  {
    pub Name: String;
    pub BasicSpriteCounter: i32;
    pub BasicSpriteFileName: Vec<String>;
    pub BasicSpriteID: Vec<i32>;
    pub BasicSpriteFileName2: Vec<String>;
    pub BasicSpriteFileName3: Vec<String>;
    pub BasicSpriteID2: Vec<i32>;
    pub BasicSpriteID3: Vec<i32>;
    pub BasicSpriteRandom: Vec<i32>;
    pub BasicPicFileName: Vec<String>;
    pub BasicPicID: Vec<i32>;
    pub PlotLast: Vec<bool>;
    pub PlotBeforeRiver: Vec<bool>;
    pub SheetFileName: String;
    pub SheetSpriteID: i32;
    pub UseSheet: bool;
    pub SidewaysSpriteFileName1: Vec<String>;
    pub SidewaysSPriteID1: Vec<i32>;
    pub SidewaysSpriteFileName2: Vec<String>;
    pub SidewaysSPriteID2: Vec<i32>;
    pub SidewaysSpriteFileName3: Vec<String>;
    pub SidewaysSPriteID3: Vec<i32>;
    pub SpecialLayer: bool;
    pub SpecialLayer6: bool;
    pub LayerSpriteID: Vec<i32>;
    pub LayerSpriteFileName: Vec<String>;
    pub OverridesCount: i32;
    pub OverridesType: Vec<i32>;
    pub OverridesZ: i32;
    pub OverridesCount2: i32;
    pub OverridesType2: Vec<i32>;
    pub MoveCost: Vec<i32>;
    pub BuildGround: i32;
    pub IsSea: bool;
    pub float[] DefBonus;
    pub float[] DefBonusMax;
    pub CanBuildRoad: bool;
    pub CanParadrop: bool;
    pub CanAmph: bool;
    pub float RoadCostModifier;
    pub OverIsTop: Vec<bool>;
    pub HidePts: i32;
    pub PreHexPicFileName: String;
    pub PreHexPicID: i32;
    pub TempHexBitmap: Bitmap;
    pub PreHexTextureFileName: String;
    pub PreHexTextureID: i32;
    pub UsePreHexTexture: bool;
    pub UsePreHexBorderTexture: bool;
    pub UsePreHexTextureAndRegularPreHex: bool;
    pub usePreHexBorderOwnZ: bool;
    pub AIBlock: i32;
    pub Red: i32;
    pub Blue: i32;
    pub Green: i32;
    pub Interior: bool;
    pub ExtraExterior: i32;
    pub ExtraExteriorSame: bool;
    pub BlackedOut: bool;
    pub PreHexBorder: i32;
    pub Description: String;
    pub DontShowInList: bool;
    pub NavyOverride: i32;
    pub AirOverride: i32;
    pub Transparent: bool;
    pub TempDefenseBonus: i32;
    pub NoPortReq: bool;
    pub Obstruction: i32;
    pub FuzzyOwnerAssured: bool;
    pub NewGfxSkyEventPic: i32;
    pub NewGfxSkyX: i32;
    pub NewGfxSkyY: i32;
    pub NewGfxBackgroundEventPic: i32;
    pub NewGfxBackgroundX: i32;
    pub NewGfxBackgroundY: i32;
    pub NewGfxForegroundEventPic: i32;
    pub NewGfxForegroundX: i32;
    pub NewGfxForegroundY: i32;
    pub NewGfxWeatherBackgroundEventPic: i32;
    pub NewGfxWeatherBackgroundX: i32;
    pub NewGfxWeatherBackgroundY: i32;
    pub NewGfxWeatherForegroundEventPic: i32;
    pub NewGfxWeatherForegroundX: i32;
    pub NewGfxWeatherForegroundY: i32;

    pub LandscapeTypeClass Clone()
    {
      BinaryFormatter binaryFormatter = BinaryFormatter::new();
      MemoryStream serializationStream = MemoryStream::new();
      binaryFormatter.Serialize((Stream) serializationStream,  this);
      serializationStream.Position = 0L;
      return (LandscapeTypeClass) binaryFormatter.Deserialize((Stream) serializationStream);
    }

    pub fn GetObjectData(SerializationInfo info, StreamingContext context)
    {
      info.AddValue("Name",  this.Name);
      info.AddValue("BasicSpriteCounter", this.BasicSpriteCounter);
      info.AddValue("BasicSpriteFileName",  this.BasicSpriteFileName);
      info.AddValue("BasicSpriteFileName2",  this.BasicSpriteFileName2);
      info.AddValue("BasicSpriteFileName3",  this.BasicSpriteFileName3);
      info.AddValue("BasicPicFileName",  this.BasicPicFileName);
      info.AddValue("SpecialLayer", this.SpecialLayer);
      info.AddValue("SpecialLayer6", this.SpecialLayer6);
      info.AddValue("LayerSpriteFileName",  this.LayerSpriteFileName);
      info.AddValue("OverridesCount", this.OverridesCount);
      info.AddValue("OverridesZ", this.OverridesZ);
      info.AddValue("OverridesType",  this.OverridesType);
      info.AddValue("OverridesCount2", this.OverridesCount2);
      info.AddValue("OverridesType2",  this.OverridesType2);
      info.AddValue("MoveCost",  this.MoveCost);
      info.AddValue("BuildGround", this.BuildGround);
      info.AddValue("PlotLast",  this.PlotLast);
      info.AddValue("IsSea", this.IsSea);
      info.AddValue("DefBonus",  this.DefBonus);
      info.AddValue("DefBonusMax",  this.DefBonusMax);
      info.AddValue("CanBuildRoad", this.CanBuildRoad);
      info.AddValue("HidePts", this.HidePts);
      info.AddValue("RoadCostModifier", this.RoadCostModifier);
      info.AddValue("PreHexPicFileName",  this.PreHexPicFileName);
      info.AddValue("AIBlock", this.AIBlock);
      info.AddValue("CanParadrop", this.CanParadrop);
      info.AddValue("BasicSpriteRandom",  this.BasicSpriteRandom);
      info.AddValue("CanAmph", this.CanAmph);
      info.AddValue("Red", this.Red);
      info.AddValue("Blue", this.Blue);
      info.AddValue("Green", this.Green);
      info.AddValue("Interior", this.Interior);
      info.AddValue("ExtraExterior", this.ExtraExterior);
      info.AddValue("ExtraExteriorSame", this.ExtraExteriorSame);
      info.AddValue("BlackedOut", this.BlackedOut);
      info.AddValue("PreHexBorder", this.PreHexBorder);
      info.AddValue("SidewaysSpriteFileName1",  this.SidewaysSpriteFileName1);
      info.AddValue("SidewaysSpriteFileName2",  this.SidewaysSpriteFileName2);
      info.AddValue("SidewaysSpriteFileName3",  this.SidewaysSpriteFileName3);
      info.AddValue("Description",  this.Description);
      info.AddValue("OverIsTop",  this.OverIsTop);
      info.AddValue("DontShowInList", this.DontShowInList);
      info.AddValue("NavyOverride", this.NavyOverride);
      info.AddValue("AirOverride", this.AirOverride);
      info.AddValue("SheetFileName",  this.SheetFileName);
      info.AddValue("UseSheet", this.UseSheet);
      info.AddValue("Transparent", this.Transparent);
      info.AddValue("NoPortReq", this.NoPortReq);
      info.AddValue("PlotBeforeRiver",  this.PlotBeforeRiver);
      info.AddValue("PreHexTextureFileName",  this.PreHexTextureFileName);
      info.AddValue("UsePreHexTexture", this.UsePreHexTexture);
      info.AddValue("UsePreHexBorderTexture", this.UsePreHexBorderTexture);
      info.AddValue("UsePreHexBorderOwnZ", this.usePreHexBorderOwnZ);
      info.AddValue("UsePreHexTextureAndRegularPreHex", this.UsePreHexTextureAndRegularPreHex);
      info.AddValue("Obstruction", this.Obstruction);
      info.AddValue("FuzzyOwnerAssured", this.FuzzyOwnerAssured);
    }

    protected LandscapeTypeClass(SerializationInfo info, StreamingContext context)
    {
      this.BasicSpriteFileName = new string[1];
      this.BasicSpriteID = new int[1];
      this.BasicSpriteFileName2 = new string[1];
      this.BasicSpriteFileName3 = new string[1];
      this.BasicSpriteID2 = new int[1];
      this.BasicSpriteID3 = new int[1];
      this.BasicSpriteRandom = new int[1];
      this.BasicPicFileName = new string[1];
      this.BasicPicID = new int[1];
      this.PlotLast = new bool[1];
      this.PlotBeforeRiver = new bool[1];
      this.SidewaysSpriteFileName1 = new string[1];
      this.SidewaysSPriteID1 = new int[1];
      this.SidewaysSpriteFileName2 = new string[1];
      this.SidewaysSPriteID2 = new int[1];
      this.SidewaysSpriteFileName3 = new string[1];
      this.SidewaysSPriteID3 = new int[1];
      this.LayerSpriteID = new int[65];
      this.LayerSpriteFileName = new string[65];
      this.OverridesType = new int[1];
      this.OverridesType2 = new int[1];
      this.MoveCost = new int[100];
      this.DefBonus = new float[100];
      this.DefBonusMax = new float[100];
      this.OverIsTop = new bool[1];
      this.Name = info.GetString(nameof (Name));
      this.BasicSpriteCounter = info.GetInt32(nameof (BasicSpriteCounter));
      this.BasicSpriteFileName = new string[this.BasicSpriteCounter + 1];
      this.BasicSpriteFileName2 = new string[this.BasicSpriteCounter + 1];
      this.BasicPicFileName = new string[this.BasicSpriteCounter + 1];
      this.BasicSpriteID = new int[this.BasicSpriteCounter + 1];
      this.BasicSpriteID2 = new int[this.BasicSpriteCounter + 1];
      this.BasicSpriteID3 = new int[this.BasicSpriteCounter + 1];
      this.BasicPicID = new int[this.BasicSpriteCounter + 1];
      this.PlotLast = new bool[this.BasicSpriteCounter + 1];
      this.BasicSpriteRandom = new int[this.BasicSpriteCounter + 1];
      this.PlotLast = (bool[]) info.GetValue(nameof (PlotLast), this.PlotLast.GetType());
      this.BasicSpriteFileName = (string[]) info.GetValue(nameof (BasicSpriteFileName), this.BasicSpriteFileName.GetType());
      this.BasicSpriteFileName2 = (string[]) info.GetValue(nameof (BasicSpriteFileName2), this.BasicSpriteFileName2.GetType());
      this.BasicPicFileName = (string[]) info.GetValue(nameof (BasicPicFileName), this.BasicPicFileName.GetType());
      this.BasicSpriteFileName3 = new string[this.BasicSpriteCounter + 1];
      if (DrawMod.TGame.Data.Version < 130)
      {
        this.OverIsTop = new bool[this.BasicSpriteCounter + 1];
        let mut basicSpriteCounter: i32 =  this.BasicSpriteCounter;
        for (let mut index: i32 =  0; index <= basicSpriteCounter; index += 1)
        {
          this.BasicSpriteRandom[index] = -1;
          this.OverIsTop[index] = false;
          this.BasicSpriteFileName3[index] = "systemgraphics/trans.bmp";
        }
        this.SpecialLayer6 = false;
      }
      else
      {
        try
        {
          this.BasicSpriteFileName3 = (string[]) info.GetValue(nameof (BasicSpriteFileName3), this.BasicSpriteFileName3.GetType());
        }
        catch (Exception ex)
        {
          ProjectData.SetProjectError(ex);
          let mut basicSpriteCounter: i32 =  this.BasicSpriteCounter;
          for (let mut index: i32 =  0; index <= basicSpriteCounter; index += 1)
            this.BasicSpriteFileName3[index] = "systemgraphics/trans.bmp";
          ProjectData.ClearProjectError();
        }
        this.OverIsTop = new bool[this.BasicSpriteCounter + 1];
        try
        {
          this.OverIsTop = (bool[]) info.GetValue(nameof (OverIsTop), this.OverIsTop.GetType());
        }
        catch (Exception ex)
        {
          ProjectData.SetProjectError(ex);
          let mut basicSpriteCounter: i32 =  this.BasicSpriteCounter;
          for (let mut index: i32 =  0; index <= basicSpriteCounter; index += 1)
            this.OverIsTop[index] = false;
          ProjectData.ClearProjectError();
        }
        try
        {
          this.BasicSpriteRandom = (int[]) info.GetValue(nameof (BasicSpriteRandom), this.BasicSpriteRandom.GetType());
        }
        catch (Exception ex)
        {
          ProjectData.SetProjectError(ex);
          let mut basicSpriteCounter: i32 =  this.BasicSpriteCounter;
          for (let mut index: i32 =  0; index <= basicSpriteCounter; index += 1)
            this.BasicSpriteRandom[index] = -1;
          ProjectData.ClearProjectError();
        }
        try
        {
          this.SpecialLayer6 = info.GetBoolean(nameof (SpecialLayer6));
        }
        catch (Exception ex)
        {
          ProjectData.SetProjectError(ex);
          this.SpecialLayer6 = false;
          ProjectData.ClearProjectError();
        }
      }
      this.SpecialLayer = info.GetBoolean(nameof (SpecialLayer));
      this.LayerSpriteFileName = (string[]) info.GetValue(nameof (LayerSpriteFileName), this.LayerSpriteFileName.GetType());
      this.OverridesCount = info.GetInt32(nameof (OverridesCount));
      this.OverridesZ = info.GetInt32(nameof (OverridesZ));
      if (this.OverridesCount > -1)
      {
        this.OverridesType = new int[this.OverridesCount + 1];
        this.OverridesType = (int[]) info.GetValue(nameof (OverridesType), this.OverridesType.GetType());
      }
      else
        this.OverridesType = new int[1];
      if (DrawMod.TGame.Data.Version < 130)
      {
        this.OverridesCount2 = -1;
        this.OverridesType2 = new int[1];
      }
      else
      {
        try
        {
          this.OverridesCount2 = info.GetInt32(nameof (OverridesCount2));
          if (this.OverridesCount2 > -1)
          {
            this.OverridesType2 = new int[this.OverridesCount2 + 1];
            this.OverridesType2 = (int[]) info.GetValue(nameof (OverridesType2), this.OverridesType2.GetType());
          }
          else
            this.OverridesType2 = new int[1];
        }
        catch (Exception ex)
        {
          ProjectData.SetProjectError(ex);
          this.OverridesType2 = new int[1];
          this.OverridesCount2 = -1;
          ProjectData.ClearProjectError();
        }
      }
      this.MoveCost = (int[]) info.GetValue(nameof (MoveCost), this.MoveCost.GetType());
      this.MoveCost = (int[]) Utils.CopyArray((Array) this.MoveCost, (Array) new int[100]);
      this.BuildGround = info.GetInt32(nameof (BuildGround));
      this.IsSea = info.GetBoolean(nameof (IsSea));
      this.DefBonus = (float[]) info.GetValue(nameof (DefBonus), this.DefBonus.GetType());
      this.DefBonusMax = (float[]) info.GetValue(nameof (DefBonusMax), this.DefBonusMax.GetType());
      this.DefBonus = (float[]) Utils.CopyArray((Array) this.DefBonus, (Array) new float[100]);
      this.DefBonusMax = (float[]) Utils.CopyArray((Array) this.DefBonusMax, (Array) new float[100]);
      this.CanBuildRoad = (uint) info.GetInt32(nameof (CanBuildRoad)) > 0U;
      this.HidePts = info.GetInt32(nameof (HidePts));
      this.RoadCostModifier = info.GetSingle(nameof (RoadCostModifier));
      this.PreHexPicFileName = info.GetString(nameof (PreHexPicFileName));
      this.AIBlock = info.GetInt32(nameof (AIBlock));
      this.CanParadrop = info.GetBoolean(nameof (CanParadrop));
      try
      {
        this.CanAmph = info.GetBoolean(nameof (CanAmph));
      }
      catch (Exception ex)
      {
        ProjectData.SetProjectError(ex);
        this.CanAmph = true;
        ProjectData.ClearProjectError();
      }
      try
      {
        this.Red = info.GetInt32(nameof (Red));
        this.Green = info.GetInt32(nameof (Green));
        this.Blue = info.GetInt32(nameof (Blue));
      }
      catch (Exception ex)
      {
        ProjectData.SetProjectError(ex);
        this.Red = -1;
        this.Green = -1;
        this.Blue = -1;
        ProjectData.ClearProjectError();
      }
      if (DrawMod.TGame.Data.Product >= 6)
      {
        try
        {
          this.Obstruction = info.GetInt32(nameof (Obstruction));
        }
        catch (Exception ex)
        {
          ProjectData.SetProjectError(ex);
          this.Obstruction = 0;
          ProjectData.ClearProjectError();
        }
        try
        {
          this.FuzzyOwnerAssured = (uint) info.GetInt32(nameof (FuzzyOwnerAssured)) > 0U;
        }
        catch (Exception ex)
        {
          ProjectData.SetProjectError(ex);
          this.FuzzyOwnerAssured = false;
          ProjectData.ClearProjectError();
        }
      }
      else
      {
        this.Obstruction = 0;
        this.FuzzyOwnerAssured = false;
      }
      if (DrawMod.TGame.Data.Version < 130)
      {
        this.DontShowInList = false;
        this.NavyOverride = -1;
        this.AirOverride = -1;
        this.Description = "";
        this.SidewaysSpriteFileName1 = new string[this.BasicSpriteCounter + 1];
        this.SidewaysSpriteFileName2 = new string[this.BasicSpriteCounter + 1];
        this.SidewaysSpriteFileName3 = new string[this.BasicSpriteCounter + 1];
        this.SidewaysSPriteID1 = new int[this.BasicSpriteCounter + 1];
        this.SidewaysSPriteID2 = new int[this.BasicSpriteCounter + 1];
        this.SidewaysSPriteID3 = new int[this.BasicSpriteCounter + 1];
        let mut basicSpriteCounter: i32 =  this.BasicSpriteCounter;
        for (let mut index: i32 =  0; index <= basicSpriteCounter; index += 1)
        {
          this.SidewaysSpriteFileName1[index] = "systemgraphics/trans.bmp";
          this.SidewaysSpriteFileName2[index] = "systemgraphics/trans.bmp";
          this.SidewaysSpriteFileName3[index] = "systemgraphics/trans.bmp";
        }
        this.PreHexBorder = -1;
        this.BlackedOut = false;
        this.ExtraExteriorSame = false;
        this.Interior = false;
        this.SheetFileName = "systemgraphics/trans.bmp";
        this.UseSheet = false;
        this.Transparent = false;
      }
      else
      {
        try
        {
          this.Interior = info.GetBoolean(nameof (Interior));
        }
        catch (Exception ex)
        {
          ProjectData.SetProjectError(ex);
          this.Interior = false;
          ProjectData.ClearProjectError();
        }
        try
        {
          this.PlotBeforeRiver = (bool[]) info.GetValue(nameof (PlotBeforeRiver), this.PlotBeforeRiver.GetType());
        }
        catch (Exception ex)
        {
          ProjectData.SetProjectError(ex);
          this.PlotBeforeRiver = this.BasicSpriteCounter < 0 ? new bool[1] : new bool[this.BasicSpriteCounter + 1];
          ProjectData.ClearProjectError();
        }
        try
        {
          this.ExtraExterior = info.GetInt32(nameof (ExtraExterior));
        }
        catch (Exception ex)
        {
          ProjectData.SetProjectError(ex);
          this.ExtraExterior = -1;
          ProjectData.ClearProjectError();
        }
        try
        {
          this.ExtraExteriorSame = info.GetBoolean(nameof (ExtraExteriorSame));
        }
        catch (Exception ex)
        {
          ProjectData.SetProjectError(ex);
          this.ExtraExteriorSame = false;
          ProjectData.ClearProjectError();
        }
        try
        {
          this.BlackedOut = info.GetBoolean(nameof (BlackedOut));
        }
        catch (Exception ex)
        {
          ProjectData.SetProjectError(ex);
          this.BlackedOut = false;
          ProjectData.ClearProjectError();
        }
        try
        {
          this.PreHexBorder = info.GetInt32(nameof (PreHexBorder));
        }
        catch (Exception ex)
        {
          ProjectData.SetProjectError(ex);
          this.PreHexBorder = -1;
          ProjectData.ClearProjectError();
        }
        this.SidewaysSpriteFileName1 = new string[this.BasicSpriteCounter + 1];
        this.SidewaysSpriteFileName2 = new string[this.BasicSpriteCounter + 1];
        this.SidewaysSpriteFileName3 = new string[this.BasicSpriteCounter + 1];
        this.SidewaysSPriteID1 = new int[this.BasicSpriteCounter + 1];
        this.SidewaysSPriteID2 = new int[this.BasicSpriteCounter + 1];
        this.SidewaysSPriteID3 = new int[this.BasicSpriteCounter + 1];
        try
        {
          this.SidewaysSpriteFileName1 = (string[]) info.GetValue(nameof (SidewaysSpriteFileName1), this.SidewaysSpriteFileName1.GetType());
          this.SidewaysSpriteFileName2 = (string[]) info.GetValue(nameof (SidewaysSpriteFileName2), this.SidewaysSpriteFileName1.GetType());
          this.SidewaysSpriteFileName3 = (string[]) info.GetValue(nameof (SidewaysSpriteFileName3), this.SidewaysSpriteFileName1.GetType());
        }
        catch (Exception ex1)
        {
          ProjectData.SetProjectError(ex1);
          try
          {
            this.SidewaysSpriteFileName1[0] = info.GetString(nameof (SidewaysSpriteFileName1));
            this.SidewaysSpriteFileName2[0] = info.GetString(nameof (SidewaysSpriteFileName2));
            this.SidewaysSpriteFileName3[0] = info.GetString(nameof (SidewaysSpriteFileName3));
            let mut basicSpriteCounter: i32 =  this.BasicSpriteCounter;
            for (let mut index: i32 =  1; index <= basicSpriteCounter; index += 1)
            {
              this.SidewaysSpriteFileName1[index] = this.SidewaysSpriteFileName1[0];
              this.SidewaysSpriteFileName2[index] = this.SidewaysSpriteFileName2[0];
              this.SidewaysSpriteFileName3[index] = this.SidewaysSpriteFileName3[0];
            }
          }
          catch (Exception ex2)
          {
            ProjectData.SetProjectError(ex2);
            let mut basicSpriteCounter: i32 =  this.BasicSpriteCounter;
            for (let mut index: i32 =  0; index <= basicSpriteCounter; index += 1)
            {
              this.SidewaysSpriteFileName1[index] = "systemgraphics/trans.bmp";
              this.SidewaysSpriteFileName2[index] = "systemgraphics/trans.bmp";
              this.SidewaysSpriteFileName3[index] = "systemgraphics/trans.bmp";
            }
            ProjectData.ClearProjectError();
          }
          ProjectData.ClearProjectError();
        }
        try
        {
          this.SheetFileName = info.GetString(nameof (SheetFileName));
          this.UseSheet = info.GetBoolean(nameof (UseSheet));
        }
        catch (Exception ex)
        {
          ProjectData.SetProjectError(ex);
          this.SheetFileName = "systemgraphics/trans.bmp";
          this.UseSheet = false;
          ProjectData.ClearProjectError();
        }
        try
        {
          this.Description = info.GetString(nameof (Description));
        }
        catch (Exception ex)
        {
          ProjectData.SetProjectError(ex);
          this.Description = "";
          ProjectData.ClearProjectError();
        }
        try
        {
          this.DontShowInList = info.GetBoolean(nameof (DontShowInList));
        }
        catch (Exception ex)
        {
          ProjectData.SetProjectError(ex);
          this.DontShowInList = false;
          ProjectData.ClearProjectError();
        }
        try
        {
          this.NavyOverride = info.GetInt32(nameof (NavyOverride));
          this.AirOverride = info.GetInt32(nameof (AirOverride));
        }
        catch (Exception ex)
        {
          ProjectData.SetProjectError(ex);
          this.NavyOverride = -1;
          this.AirOverride = -1;
          ProjectData.ClearProjectError();
        }
        try
        {
          this.Transparent = info.GetBoolean(nameof (Transparent));
        }
        catch (Exception ex)
        {
          ProjectData.SetProjectError(ex);
          this.Transparent = false;
          ProjectData.ClearProjectError();
        }
        try
        {
          this.NoPortReq = info.GetBoolean(nameof (NoPortReq));
        }
        catch (Exception ex)
        {
          ProjectData.SetProjectError(ex);
          this.NoPortReq = false;
          ProjectData.ClearProjectError();
        }
        try
        {
          this.PreHexTextureFileName = info.GetString(nameof (PreHexTextureFileName));
          this.UsePreHexTexture = info.GetBoolean(nameof (UsePreHexTexture));
        }
        catch (Exception ex)
        {
          ProjectData.SetProjectError(ex);
          this.PreHexTextureFileName = "systemgraphics/trans.bmp";
          this.PreHexTextureID = -1;
          this.UsePreHexTexture = false;
          ProjectData.ClearProjectError();
        }
        try
        {
          this.UsePreHexBorderTexture = info.GetBoolean(nameof (UsePreHexBorderTexture));
        }
        catch (Exception ex)
        {
          ProjectData.SetProjectError(ex);
          this.UsePreHexBorderTexture = false;
          ProjectData.ClearProjectError();
        }
        try
        {
          this.usePreHexBorderOwnZ = info.GetBoolean("UsePreHexBorderOwnZ");
        }
        catch (Exception ex)
        {
          ProjectData.SetProjectError(ex);
          this.usePreHexBorderOwnZ = false;
          ProjectData.ClearProjectError();
        }
        try
        {
          this.UsePreHexTextureAndRegularPreHex = info.GetBoolean(nameof (UsePreHexTextureAndRegularPreHex));
        }
        catch (Exception ex)
        {
          ProjectData.SetProjectError(ex);
          this.UsePreHexTextureAndRegularPreHex = false;
          ProjectData.ClearProjectError();
        }
        this.NewGfxSkyEventPic = -1;
        this.NewGfxSkyX = -1;
        this.NewGfxSkyY = -1;
        this.NewGfxBackgroundEventPic = -1;
        this.NewGfxBackgroundX = -1;
        this.NewGfxBackgroundY = -1;
        this.NewGfxForegroundEventPic = -1;
        this.NewGfxForegroundX = -1;
        this.NewGfxForegroundY = -1;
        this.NewGfxWeatherBackgroundEventPic = -1;
        this.NewGfxWeatherBackgroundX = -1;
        this.NewGfxWeatherBackgroundY = -1;
        this.NewGfxWeatherForegroundEventPic = -1;
        this.NewGfxWeatherForegroundX = -1;
        this.NewGfxWeatherForegroundY = -1;
      }
    }

    pub CheckOverride: bool(lt: i32)
    {
      if (this.OverridesCount <= -1)
        return false;
      let mut num: i32 =  0;
      let mut overridesCount: i32 =  this.OverridesCount;
      for (let mut index: i32 =  0; index <= overridesCount; index += 1)
      {
        if (this.OverridesType[index] == lt)
          num = 1;
      }
      return num == 1;
    }

    pub CheckOverride2: bool(lt: i32)
    {
      if (this.OverridesCount2 <= -1)
        return false;
      let mut num: i32 =  0;
      let mut overridesCount2: i32 =  this.OverridesCount2;
      for (let mut index: i32 =  0; index <= overridesCount2; index += 1)
      {
        if (this.OverridesType2[index] == lt)
          num = 1;
      }
      return num == 1;
    }

    pub fn AddOverride(lt: i32)
    {
      if (this.OverridesCount < -1)
        this.OverridesCount = -1;
      this += 1.OverridesCount;
      this.OverridesType = (int[]) Utils.CopyArray((Array) this.OverridesType, (Array) new int[this.OverridesCount + 1]);
      this.OverridesType[this.OverridesCount] = lt;
    }

    pub fn RemoveOverride(nr: i32)
    {
      if (nr < this.OverridesCount)
      {
        let mut num1: i32 =  nr;
        let mut num2: i32 =  this.OverridesCount - 1;
        for (let mut index: i32 =  num1; index <= num2; index += 1)
          this.OverridesType[index] = this.OverridesType[index + 1];
      }
      --this.OverridesCount;
      if (this.OverridesCount <= -1)
        return;
      this.OverridesType = (int[]) Utils.CopyArray((Array) this.OverridesType, (Array) new int[this.OverridesCount + 1]);
    }

    pub fn AddOverride2(lt: i32)
    {
      if (this.OverridesCount2 < -1)
        this.OverridesCount2 = -1;
      this += 1.OverridesCount2;
      this.OverridesType2 = (int[]) Utils.CopyArray((Array) this.OverridesType2, (Array) new int[this.OverridesCount2 + 1]);
      this.OverridesType2[this.OverridesCount2] = lt;
    }

    pub fn RemoveOverride2(nr: i32)
    {
      if (nr < this.OverridesCount2)
      {
        let mut num1: i32 =  nr;
        let mut num2: i32 =  this.OverridesCount2 - 1;
        for (let mut index: i32 =  num1; index <= num2; index += 1)
          this.OverridesType2[index] = this.OverridesType2[index + 1];
      }
      --this.OverridesCount2;
      if (this.OverridesCount2 <= -1)
        return;
      this.OverridesType2 = (int[]) Utils.CopyArray((Array) this.OverridesType2, (Array) new int[this.OverridesCount2 + 1]);
    }

    pub LandscapeTypeClass(hardcoded: i32)
    {
      this.BasicSpriteFileName = new string[1];
      this.BasicSpriteID = new int[1];
      this.BasicSpriteFileName2 = new string[1];
      this.BasicSpriteFileName3 = new string[1];
      this.BasicSpriteID2 = new int[1];
      this.BasicSpriteID3 = new int[1];
      this.BasicSpriteRandom = new int[1];
      this.BasicPicFileName = new string[1];
      this.BasicPicID = new int[1];
      this.PlotLast = new bool[1];
      this.PlotBeforeRiver = new bool[1];
      this.SidewaysSpriteFileName1 = new string[1];
      this.SidewaysSPriteID1 = new int[1];
      this.SidewaysSpriteFileName2 = new string[1];
      this.SidewaysSPriteID2 = new int[1];
      this.SidewaysSpriteFileName3 = new string[1];
      this.SidewaysSPriteID3 = new int[1];
      this.LayerSpriteID = new int[65];
      this.LayerSpriteFileName = new string[65];
      this.OverridesType = new int[1];
      this.OverridesType2 = new int[1];
      this.MoveCost = new int[100];
      this.DefBonus = new float[100];
      this.DefBonusMax = new float[100];
      this.OverIsTop = new bool[1];
      this.ExtraExterior = -1;
      this.ExtraExteriorSame = false;
      this.OverridesCount2 = -1;
      this.FuzzyOwnerAssured = false;
      this.NavyOverride = -1;
      this.AirOverride = -1;
      this.PreHexBorder = -1;
      this.SidewaysSpriteFileName1[0] = "systemgraphics/trans.bmp";
      this.SidewaysSpriteFileName2[0] = "systemgraphics/trans.bmp";
      this.SidewaysSpriteFileName3[0] = "systemgraphics/trans.bmp";
      if (hardcoded == 0)
      {
        this.SidewaysSpriteFileName1[0] = "systemgraphics/trans.bmp";
        this.SidewaysSpriteFileName2[0] = "systemgraphics/trans.bmp";
        this.SidewaysSpriteFileName3[0] = "systemgraphics/trans.bmp";
        this.Name = "Default Land";
        this.BasicSpriteCounter = 0;
        this.BasicSpriteFileName[0] = "systemgraphics/trans.bmp";
        this.BasicSpriteFileName2[0] = "systemgraphics/trans.bmp";
        this.BasicSpriteFileName3[0] = "systemgraphics/trans.bmp";
        this.BasicPicFileName[0] = "systemgraphics/trans.bmp";
        this.PreHexPicFileName = "systemgraphics/blackhex.bmp";
        this.BasicSpriteRandom[0] = -1;
        this.BasicSpriteID[0] = 0;
        this.SpecialLayer = false;
        let mut index: i32 =  0;
        do
        {
          this.MoveCost[index] = 9999;
          index += 1;
        }
        while (index <= 19);
        this.CanParadrop = true;
      }
      if (hardcoded == 1)
      {
        this.Name = "Sea".to_owned();
        this.CanParadrop = true;
        this.BasicSpriteCounter = 0;
        this.BasicSpriteFileName[0] = "systemgraphics/trans.bmp";
        this.BasicSpriteFileName2[0] = "systemgraphics/trans.bmp";
        this.BasicSpriteFileName3[0] = "systemgraphics/trans.bmp";
        this.BasicPicFileName[0] = "systemgraphics/trans.bmp";
        this.PreHexPicFileName = "systemgraphics/whitehex.bmp";
        this.BasicSpriteRandom[0] = -1;
        this.BasicSpriteID[0] = 0;
        this.SpecialLayer = true;
        let mut index1: i32 =  1;
        do
        {
          this.LayerSpriteFileName[index1] = "systemgraphics/trans.bmp";
          index1 += 1;
        }
        while (index1 <= 64);
        this.OverridesCount = 0;
        this.OverridesType[0] = 0;
        this.NavyOverride = -1;
        this.AirOverride = -1;
        this.OverridesZ = 100;
        this.RoadCostModifier = 1f;
        let mut index2: i32 =  0;
        do
        {
          this.MoveCost[index2] = 999;
          index2 += 1;
        }
        while (index2 <= 19);
      }
      this.SheetFileName = "systemgraphics/trans.bmp";
      this.UseSheet = false;
      this.NoPortReq = false;
      this.PreHexTextureFileName = "systemgraphics/trans.bmp";
      this.PreHexTextureID = -1;
      this.UsePreHexTexture = false;
      this.UsePreHexBorderTexture = false;
      this.usePreHexBorderOwnZ = false;
      this.NewGfxSkyEventPic = -1;
      this.NewGfxSkyX = -1;
      this.NewGfxSkyY = -1;
      this.NewGfxBackgroundEventPic = -1;
      this.NewGfxBackgroundX = -1;
      this.NewGfxBackgroundY = -1;
      this.NewGfxForegroundEventPic = -1;
      this.NewGfxForegroundX = -1;
      this.NewGfxForegroundY = -1;
      this.NewGfxWeatherBackgroundEventPic = -1;
      this.NewGfxWeatherBackgroundX = -1;
      this.NewGfxWeatherBackgroundY = -1;
      this.NewGfxWeatherForegroundEventPic = -1;
      this.NewGfxWeatherForegroundX = -1;
      this.NewGfxWeatherForegroundY = -1;
    }

    pub fn AutoLoadSpecial(dirstring: String, extstring: String)
    {
      this.LayerSpriteFileName[1] = dirstring + "/a1" + extstring;
      this.LayerSpriteFileName[2] = dirstring + "/b1" + extstring;
      this.LayerSpriteFileName[3] = dirstring + "/b2" + extstring;
      this.LayerSpriteFileName[4] = dirstring + "/b3" + extstring;
      this.LayerSpriteFileName[5] = dirstring + "/b4" + extstring;
      this.LayerSpriteFileName[6] = dirstring + "/b5" + extstring;
      this.LayerSpriteFileName[7] = dirstring + "/b6" + extstring;
      this.LayerSpriteFileName[8] = dirstring + "/c1" + extstring;
      this.LayerSpriteFileName[9] = dirstring + "/c2" + extstring;
      this.LayerSpriteFileName[10] = dirstring + "/c3" + extstring;
      this.LayerSpriteFileName[11] = dirstring + "/c4" + extstring;
      this.LayerSpriteFileName[12] = dirstring + "/c5" + extstring;
      this.LayerSpriteFileName[13] = dirstring + "/c6" + extstring;
      this.LayerSpriteFileName[14] = dirstring + "/c7" + extstring;
      this.LayerSpriteFileName[15] = dirstring + "/c8" + extstring;
      this.LayerSpriteFileName[16] = dirstring + "/c9" + extstring;
      this.LayerSpriteFileName[17] = dirstring + "/c10" + extstring;
      this.LayerSpriteFileName[18] = dirstring + "/c11" + extstring;
      this.LayerSpriteFileName[19] = dirstring + "/c12" + extstring;
      this.LayerSpriteFileName[20] = dirstring + "/c13" + extstring;
      this.LayerSpriteFileName[21] = dirstring + "/c14" + extstring;
      this.LayerSpriteFileName[22] = dirstring + "/c15" + extstring;
      this.LayerSpriteFileName[23] = dirstring + "/d1" + extstring;
      this.LayerSpriteFileName[24] = dirstring + "/d2" + extstring;
      this.LayerSpriteFileName[25] = dirstring + "/d3" + extstring;
      this.LayerSpriteFileName[26] = dirstring + "/d4" + extstring;
      this.LayerSpriteFileName[27] = dirstring + "/d5" + extstring;
      this.LayerSpriteFileName[28] = dirstring + "/d6" + extstring;
      this.LayerSpriteFileName[29] = dirstring + "/d7" + extstring;
      this.LayerSpriteFileName[30] = dirstring + "/d8" + extstring;
      this.LayerSpriteFileName[31] = dirstring + "/d9" + extstring;
      this.LayerSpriteFileName[32] = dirstring + "/d10" + extstring;
      this.LayerSpriteFileName[33] = dirstring + "/d11" + extstring;
      this.LayerSpriteFileName[34] = dirstring + "/d12" + extstring;
      this.LayerSpriteFileName[35] = dirstring + "/d13" + extstring;
      this.LayerSpriteFileName[36] = dirstring + "/d14" + extstring;
      this.LayerSpriteFileName[37] = dirstring + "/d15" + extstring;
      this.LayerSpriteFileName[38] = dirstring + "/d16" + extstring;
      this.LayerSpriteFileName[39] = dirstring + "/d17" + extstring;
      this.LayerSpriteFileName[40] = dirstring + "/d18" + extstring;
      this.LayerSpriteFileName[41] = dirstring + "/d19" + extstring;
      this.LayerSpriteFileName[42] = dirstring + "/d20" + extstring;
      this.LayerSpriteFileName[43] = dirstring + "/e1" + extstring;
      this.LayerSpriteFileName[44] = dirstring + "/e2" + extstring;
      this.LayerSpriteFileName[45] = dirstring + "/e3" + extstring;
      this.LayerSpriteFileName[46] = dirstring + "/e4" + extstring;
      this.LayerSpriteFileName[47] = dirstring + "/e5" + extstring;
      this.LayerSpriteFileName[48] = dirstring + "/e6" + extstring;
      this.LayerSpriteFileName[49] = dirstring + "/e7" + extstring;
      this.LayerSpriteFileName[50] = dirstring + "/e8" + extstring;
      this.LayerSpriteFileName[51] = dirstring + "/e9" + extstring;
      this.LayerSpriteFileName[52] = dirstring + "/e10" + extstring;
      this.LayerSpriteFileName[53] = dirstring + "/e11" + extstring;
      this.LayerSpriteFileName[54] = dirstring + "/e12" + extstring;
      this.LayerSpriteFileName[55] = dirstring + "/e13" + extstring;
      this.LayerSpriteFileName[56] = dirstring + "/e14" + extstring;
      this.LayerSpriteFileName[57] = dirstring + "/e15" + extstring;
      this.LayerSpriteFileName[58] = dirstring + "/f1" + extstring;
      this.LayerSpriteFileName[59] = dirstring + "/f2" + extstring;
      this.LayerSpriteFileName[60] = dirstring + "/f3" + extstring;
      this.LayerSpriteFileName[61] = dirstring + "/f4" + extstring;
      this.LayerSpriteFileName[62] = dirstring + "/f5" + extstring;
      this.LayerSpriteFileName[63] = dirstring + "/f6" + extstring;
      this.LayerSpriteFileName[64] = dirstring + "/g1" + extstring;
      this.ReloadSpecialSprites();
    }

    pub fn Kill()
    {
      if (this.BasicSpriteCounter > -1)
      {
        let mut basicSpriteCounter: i32 =  this.BasicSpriteCounter;
        for (let mut index: i32 =  0; index <= basicSpriteCounter; index += 1)
        {
          BitmapStore.RemoveBitmapNr(this.BasicSpriteID[index]);
          BitmapStore.RemoveBitmapNr(this.BasicPicID[index]);
          BitmapStore.RemoveBitmapNr(this.SidewaysSPriteID1[index]);
          BitmapStore.RemoveBitmapNr(this.SidewaysSPriteID2[index]);
          BitmapStore.RemoveBitmapNr(this.SidewaysSPriteID3[index]);
        }
      }
      if (this.SpecialLayer && !this.UseSheet)
      {
        let mut index: i32 =  1;
        do
        {
          BitmapStore.RemoveBitmapNr(this.LayerSpriteID[index]);
          index += 1;
        }
        while (index <= 64);
      }
      BitmapStore.RemoveBitmapNr(this.SheetSpriteID);
    }

    pub fn AddBasicSprite(filename: String, picfilename: String)
    {
      this += 1.BasicSpriteCounter;
      this.BasicSpriteFileName = (string[]) Utils.CopyArray((Array) this.BasicSpriteFileName, (Array) new string[this.BasicSpriteCounter + 1]);
      this.BasicSpriteID = (int[]) Utils.CopyArray((Array) this.BasicSpriteID, (Array) new int[this.BasicSpriteCounter + 1]);
      this.BasicSpriteFileName2 = (string[]) Utils.CopyArray((Array) this.BasicSpriteFileName2, (Array) new string[this.BasicSpriteCounter + 1]);
      this.BasicSpriteID2 = (int[]) Utils.CopyArray((Array) this.BasicSpriteID2, (Array) new int[this.BasicSpriteCounter + 1]);
      this.BasicSpriteFileName3 = (string[]) Utils.CopyArray((Array) this.BasicSpriteFileName3, (Array) new string[this.BasicSpriteCounter + 1]);
      this.BasicSpriteID3 = (int[]) Utils.CopyArray((Array) this.BasicSpriteID3, (Array) new int[this.BasicSpriteCounter + 1]);
      this.BasicPicFileName = (string[]) Utils.CopyArray((Array) this.BasicPicFileName, (Array) new string[this.BasicSpriteCounter + 1]);
      this.BasicPicID = (int[]) Utils.CopyArray((Array) this.BasicPicID, (Array) new int[this.BasicSpriteCounter + 1]);
      this.PlotLast = (bool[]) Utils.CopyArray((Array) this.PlotLast, (Array) new bool[this.BasicSpriteCounter + 1]);
      this.PlotBeforeRiver = (bool[]) Utils.CopyArray((Array) this.PlotBeforeRiver, (Array) new bool[this.BasicSpriteCounter + 1]);
      this.BasicSpriteRandom = (int[]) Utils.CopyArray((Array) this.BasicSpriteRandom, (Array) new int[this.BasicSpriteCounter + 1]);
      this.OverIsTop = (bool[]) Utils.CopyArray((Array) this.OverIsTop, (Array) new bool[this.BasicSpriteCounter + 1]);
      this.SidewaysSpriteFileName1 = (string[]) Utils.CopyArray((Array) this.SidewaysSpriteFileName1, (Array) new string[this.BasicSpriteCounter + 1]);
      this.SidewaysSpriteFileName2 = (string[]) Utils.CopyArray((Array) this.SidewaysSpriteFileName2, (Array) new string[this.BasicSpriteCounter + 1]);
      this.SidewaysSpriteFileName3 = (string[]) Utils.CopyArray((Array) this.SidewaysSpriteFileName3, (Array) new string[this.BasicSpriteCounter + 1]);
      this.SidewaysSPriteID1 = (int[]) Utils.CopyArray((Array) this.SidewaysSPriteID1, (Array) new int[this.BasicSpriteCounter + 1]);
      this.SidewaysSPriteID2 = (int[]) Utils.CopyArray((Array) this.SidewaysSPriteID2, (Array) new int[this.BasicSpriteCounter + 1]);
      this.SidewaysSPriteID3 = (int[]) Utils.CopyArray((Array) this.SidewaysSPriteID3, (Array) new int[this.BasicSpriteCounter + 1]);
      this.BasicSpriteFileName[this.BasicSpriteCounter] = filename;
      this.BasicSpriteID[this.BasicSpriteCounter] = BitmapStore.AddFile(this.BasicSpriteFileName[this.BasicSpriteCounter], false, true);
      this.BasicSpriteFileName2[this.BasicSpriteCounter] = "systemgraphics/trans.bmp";
      this.BasicSpriteID2[this.BasicSpriteCounter] = BitmapStore.AddFile(this.BasicSpriteFileName2[this.BasicSpriteCounter], false, true);
      this.BasicSpriteFileName3[this.BasicSpriteCounter] = "systemgraphics/trans.bmp";
      this.BasicSpriteID3[this.BasicSpriteCounter] = BitmapStore.AddFile(this.BasicSpriteFileName3[this.BasicSpriteCounter], false, true);
      this.SidewaysSpriteFileName1[this.BasicSpriteCounter] = "systemgraphics/trans.bmp";
      this.SidewaysSPriteID1[this.BasicSpriteCounter] = BitmapStore.AddFile(this.SidewaysSpriteFileName1[this.BasicSpriteCounter], false, true);
      this.SidewaysSpriteFileName2[this.BasicSpriteCounter] = "systemgraphics/trans.bmp";
      this.SidewaysSPriteID2[this.BasicSpriteCounter] = BitmapStore.AddFile(this.SidewaysSpriteFileName2[this.BasicSpriteCounter], false, true);
      this.SidewaysSpriteFileName3[this.BasicSpriteCounter] = "systemgraphics/trans.bmp";
      this.SidewaysSPriteID3[this.BasicSpriteCounter] = BitmapStore.AddFile(this.SidewaysSpriteFileName3[this.BasicSpriteCounter], false, true);
      this.BasicPicFileName[this.BasicSpriteCounter] = picfilename;
      this.OverIsTop[this.BasicSpriteCounter] = false;
      this.BasicPicID[this.BasicSpriteCounter] = BitmapStore.AddFile(this.BasicPicFileName[this.BasicSpriteCounter], false);
    }

    pub fn ReplaceSpriteSheet(filename: String)
    {
      this.SheetFileName = filename;
      this.SheetSpriteID = BitmapStore.ReloadFile(this.SheetSpriteID, this.SheetFileName, IsBig: true);
    }

    pub fn ReplaceBasicSprite(nr: i32, filename: String)
    {
      this.BasicSpriteFileName[nr] = filename;
      this.BasicSpriteID[nr] = BitmapStore.ReloadFile(this.BasicSpriteID[nr], filename, IsBig: true);
    }

    pub fn ReplaceBasicSprite2(nr: i32, filename: String)
    {
      this.BasicSpriteFileName2[nr] = filename;
      this.BasicSpriteID2[nr] = BitmapStore.ReloadFile(this.BasicSpriteID2[nr], filename, IsBig: true);
    }

    pub fn ReplaceBasicSprite3(nr: i32, filename: String)
    {
      this.BasicSpriteFileName3[nr] = filename;
      this.BasicSpriteID3[nr] = BitmapStore.ReloadFile(this.BasicSpriteID3[nr], filename, IsBig: true);
    }

    pub fn ReplacePreHexTexture(filename: String)
    {
      this.PreHexTextureFileName = filename;
      this.PreHexTextureID = BitmapStore.ReloadFile(this.PreHexTextureID, filename, IsBig: true);
      BitmapStore.simpleByteCacheObj[this.PreHexTextureID] = (SimpleByteCache) null;
      BitmapStore.simpleByteCacheSet[this.PreHexTextureID] = false;
    }

    pub fn ReplacePicSprite(nr: i32, filename: String)
    {
      this.BasicPicFileName[nr] = filename;
      this.BasicPicID[nr] = BitmapStore.ReloadFile(this.BasicPicID[nr], filename);
    }

    pub fn ReplaceSidewaysSprite1(filename: String, nr: i32)
    {
      this.SidewaysSpriteFileName1[nr] = filename;
      this.SidewaysSPriteID1[nr] = BitmapStore.ReloadFile(this.SidewaysSPriteID1[nr], this.SidewaysSpriteFileName1[nr]);
    }

    pub fn ReplaceSidewaysSprite2(filename: String, nr: i32)
    {
      this.SidewaysSpriteFileName2[nr] = filename;
      this.SidewaysSPriteID2[nr] = BitmapStore.ReloadFile(this.SidewaysSPriteID2[nr], this.SidewaysSpriteFileName2[nr]);
    }

    pub fn ReplaceSidewaysSprite3(filename: String, nr: i32)
    {
      this.SidewaysSpriteFileName3[nr] = filename;
      this.SidewaysSPriteID3[nr] = BitmapStore.ReloadFile(this.SidewaysSPriteID3[nr], this.SidewaysSpriteFileName3[nr]);
    }

    pub fn ReplacePreHexPicSprite(filename: String)
    {
      this.PreHexPicFileName = filename;
      this.PreHexPicID = BitmapStore.ReloadFile(this.PreHexPicID, filename, IsBig: true);
    }

    pub fn ReplaceSpecialSprite(nr: i32, filename: String)
    {
      this.LayerSpriteFileName[nr] = filename;
      this.LayerSpriteID[nr] = BitmapStore.ReloadFile(this.LayerSpriteID[nr], filename, IsBig: true);
    }

    pub fn LoadSprites()
    {
      if (this.BasicSpriteCounter > -1)
      {
        let mut basicSpriteCounter: i32 =  this.BasicSpriteCounter;
        for (let mut index: i32 =  0; index <= basicSpriteCounter; index += 1)
        {
          this.BasicSpriteID[index] = BitmapStore.AddFile(this.BasicSpriteFileName[index], false, true);
          this.BasicSpriteID2[index] = BitmapStore.AddFile(this.BasicSpriteFileName2[index], false, true);
          this.BasicSpriteID3[index] = BitmapStore.AddFile(this.BasicSpriteFileName3[index], false, true);
          this.BasicPicID[index] = BitmapStore.AddFile(this.BasicPicFileName[index], false);
          this.SidewaysSPriteID1[index] = BitmapStore.AddFile(this.SidewaysSpriteFileName1[index], false);
          this.SidewaysSPriteID2[index] = BitmapStore.AddFile(this.SidewaysSpriteFileName2[index], false);
          this.SidewaysSPriteID3[index] = BitmapStore.AddFile(this.SidewaysSpriteFileName3[index], false);
        }
      }
      if (this.SpecialLayer && !this.UseSheet)
      {
        let mut index: i32 =  1;
        do
        {
          this.LayerSpriteID[index] = BitmapStore.AddFile(this.LayerSpriteFileName[index], false, true);
          index += 1;
        }
        while (index <= 64);
      }
      this.PreHexPicID = BitmapStore.AddFile(this.PreHexPicFileName, false, true);
      this.SheetSpriteID = BitmapStore.AddFile(this.SheetFileName, false, true);
      this.PreHexTextureID = BitmapStore.AddFile(this.PreHexTextureFileName, false, true);
    }

    pub fn ReloadSpecialSprites()
    {
      let mut index: i32 =  1;
      do
      {
        this.LayerSpriteID[index] = this.LayerSpriteID[index] <= 0 ? BitmapStore.AddFile(this.LayerSpriteFileName[index], false, true) : BitmapStore.ReloadFile(this.LayerSpriteID[index], this.LayerSpriteFileName[index], IsBig: true);
        index += 1;
      }
      while (index <= 64);
    }
  }
}
