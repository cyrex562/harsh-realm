// Decompiled with JetBrains decompiler
// Type: WindowsApplication1.PeopleClass
// Assembly: WindowsApplication1, Version=1.0.8020.28903, Culture=neutral, PublicKeyToken=null
// MVID: F52869E5-0850-48AD-BBBE-68E7A4900AFE
// Assembly location: C:\Program Files (x86)\Steam\steamapps\common\Shadow Empire\ShadowEmpire.exe

// usingMicrosoft.VisualBasic.CompilerServices;
// usingSystem;
// usingSystem.IO;
// usingSystem.Runtime.Serialization;
// usingSystem.Runtime.Serialization.Formatters.Binary;

namespace WindowsApplication1
{
  [Serializable]
  pub class PeopleClass : ISerializable
  {
    pub Name: String;
    pub PeopleGroup: i32;
    pub BaseMorale: Vec<i32>;
    pub float[] ProdMod;
    pub float[] ProdMod2;
    pub float[] ProdMod3;
    pub float[] ProdMod4;
    pub float[] BattleForMod;
    pub float[] BattleVSMod;
    pub PolPts: i32;
    pub RegCol: i32;
    pub Red: i32;
    pub Blue: i32;
    pub Green: i32;
    pub artCode: i32;
    pub Description: String;
    pub BreakAt: i32;
    pub ExtraGraphicUse: i32;
    pub SidewaysFileName: String;
    pub SidewaysSpriteID: i32;
    pub SidewaysFileName2: String;
    pub SidewaysSpriteID2: i32;
    pub SidewaysFileName3: String;
    pub SidewaysSpriteID3: i32;
    pub SidewaysFileName4: String;
    pub SidewaysSpriteID4: i32;
    pub NationalFileName: String;
    pub NationalSpriteID: i32;
    pub SFIll: i32;
    pub SFExtra: i32;
    pub LibIdClass LibId;
    pub id: i32;
    pub tv0: i32;
    pub tv1: i32;
    pub tv2: i32;

    pub fn GetObjectData(SerializationInfo info, StreamingContext context)
    {
      info.AddValue("Name",  this.Name);
      info.AddValue("PeopleGroup", this.PeopleGroup);
      info.AddValue("BaseMorale",  this.BaseMorale);
      info.AddValue("ProdMod",  this.ProdMod);
      info.AddValue("ProdMod2",  this.ProdMod2);
      info.AddValue("ProdMod3",  this.ProdMod3);
      info.AddValue("ProdMod4",  this.ProdMod4);
      info.AddValue("PolPts", this.PolPts);
      info.AddValue("BattleForMod",  this.BattleForMod);
      info.AddValue("BattleVSMod",  this.BattleVSMod);
      info.AddValue("RegCol", this.RegCol);
      info.AddValue("Red", this.Red);
      info.AddValue("Blue", this.Blue);
      info.AddValue("Green", this.Green);
      info.AddValue("BreakAt", this.BreakAt);
      info.AddValue("ExtraGraphicUse", this.ExtraGraphicUse);
      info.AddValue("SidewaysFileName",  this.SidewaysFileName);
      info.AddValue("NationalFileName",  this.NationalFileName);
      info.AddValue("Description",  this.Description);
      info.AddValue("SFIll", this.SFIll);
      info.AddValue("SFExtra", this.SFExtra);
      info.AddValue("LibId",  this.LibId);
      info.AddValue("Id", this.id);
      info.AddValue("SidewaysFileName2",  this.SidewaysFileName2);
      info.AddValue("SidewaysFileName3",  this.SidewaysFileName3);
      info.AddValue("SidewaysFileName4",  this.SidewaysFileName4);
      info.AddValue("tv0", this.tv0);
      info.AddValue("tv1", this.tv1);
      info.AddValue("tv2", this.tv2);
      info.AddValue("artCode", this.artCode);
    }

    pub PeopleClass Clone()
    {
      BinaryFormatter binaryFormatter = BinaryFormatter::new();
      MemoryStream serializationStream = MemoryStream::new();
      binaryFormatter.Serialize((Stream) serializationStream,  this);
      serializationStream.Position = 0L;
      return (PeopleClass) binaryFormatter.Deserialize((Stream) serializationStream);
    }

    protected PeopleClass(SerializationInfo info, StreamingContext context)
    {
      this.BaseMorale = new int[100];
      this.ProdMod = new float[100];
      this.ProdMod2 = new float[100];
      this.ProdMod3 = new float[100];
      this.ProdMod4 = new float[100];
      this.BattleForMod = new float[100];
      this.BattleVSMod = new float[100];
      this.Name = info.GetString(nameof (Name));
      this.PeopleGroup = info.GetInt32(nameof (PeopleGroup));
      this.BaseMorale = (int[]) info.GetValue(nameof (BaseMorale), this.BaseMorale.GetType());
      this.BaseMorale = (int[]) Utils.CopyArray((Array) this.BaseMorale, (Array) new int[100]);
      this.ProdMod = (float[]) info.GetValue(nameof (ProdMod), this.ProdMod.GetType());
      this.ProdMod = (float[]) Utils.CopyArray((Array) this.ProdMod, (Array) new float[100]);
      try
      {
        this.ProdMod2 = (float[]) info.GetValue(nameof (ProdMod2), this.ProdMod2.GetType());
        this.ProdMod3 = (float[]) info.GetValue(nameof (ProdMod3), this.ProdMod3.GetType());
        this.ProdMod4 = (float[]) info.GetValue(nameof (ProdMod4), this.ProdMod4.GetType());
      }
      catch (Exception ex)
      {
        ProjectData.SetProjectError(ex);
        let mut index: i32 =  0;
        do
        {
          this.ProdMod2[index] = 1f;
          this.ProdMod3[index] = 1f;
          this.ProdMod4[index] = 1f;
          index += 1;
        }
        while (index <= 99);
        ProjectData.ClearProjectError();
      }
      this.PolPts = info.GetInt32(nameof (PolPts));
      this.BattleForMod = (float[]) info.GetValue(nameof (BattleForMod), this.BattleForMod.GetType());
      this.BattleForMod = (float[]) Utils.CopyArray((Array) this.BattleForMod, (Array) new float[100]);
      this.BattleVSMod = (float[]) info.GetValue(nameof (BattleVSMod), this.BattleVSMod.GetType());
      this.BattleVSMod = (float[]) Utils.CopyArray((Array) this.BattleVSMod, (Array) new float[100]);
      this.RegCol = info.GetInt32(nameof (RegCol));
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
      this.BreakAt = info.GetInt32(nameof (BreakAt));
      try
      {
        this.ExtraGraphicUse = info.GetInt32(nameof (ExtraGraphicUse));
      }
      catch (Exception ex)
      {
        ProjectData.SetProjectError(ex);
        this.ExtraGraphicUse = -1;
        ProjectData.ClearProjectError();
      }
      try
      {
        this.SidewaysFileName = info.GetString(nameof (SidewaysFileName));
      }
      catch (Exception ex)
      {
        ProjectData.SetProjectError(ex);
        this.SidewaysFileName = "systemgraphics/trans.bmp";
        ProjectData.ClearProjectError();
      }
      try
      {
        this.NationalFileName = info.GetString(nameof (NationalFileName));
      }
      catch (Exception ex)
      {
        ProjectData.SetProjectError(ex);
        this.NationalFileName = "systemgraphics/trans.bmp";
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
        this.SFIll = info.GetInt32(nameof (SFIll));
        this.SFExtra = info.GetInt32(nameof (SFExtra));
      }
      catch (Exception ex)
      {
        ProjectData.SetProjectError(ex);
        this.SFIll = -1;
        this.SFExtra = -1;
        ProjectData.ClearProjectError();
      }
      try
      {
        this.LibId = LibIdClass::new();
        this.LibId = (LibIdClass) info.GetValue(nameof (LibId), this.LibId.GetType());
      }
      catch (Exception ex)
      {
        ProjectData.SetProjectError(ex);
        this.LibId = LibIdClass::new();
        ProjectData.ClearProjectError();
      }
      try
      {
        this.id = info.GetInt32("Id");
      }
      catch (Exception ex)
      {
        ProjectData.SetProjectError(ex);
        this.id = -1;
        ProjectData.ClearProjectError();
      }
      try
      {
        this.SidewaysFileName2 = info.GetString(nameof (SidewaysFileName2));
        this.SidewaysFileName3 = info.GetString(nameof (SidewaysFileName3));
        this.SidewaysFileName4 = info.GetString(nameof (SidewaysFileName4));
        this.tv0 = info.GetInt32(nameof (tv0));
        this.tv1 = info.GetInt32(nameof (tv1));
        this.tv2 = info.GetInt32(nameof (tv2));
      }
      catch (Exception ex)
      {
        ProjectData.SetProjectError(ex);
        this.SidewaysFileName2 = "systemgraphics/trans.bmp";
        this.SidewaysFileName3 = "systemgraphics/trans.bmp";
        this.SidewaysFileName4 = "systemgraphics/trans.bmp";
        this.tv0 = 0;
        this.tv1 = 0;
        this.tv2 = 0;
        ProjectData.ClearProjectError();
      }
      try
      {
        this.artCode = info.GetInt32(nameof (artCode));
      }
      catch (Exception ex)
      {
        ProjectData.SetProjectError(ex);
        this.artCode = 0;
        ProjectData.ClearProjectError();
      }
    }

    pub PeopleClass(hardcoded: i32)
    {
      this.BaseMorale = new int[100];
      this.ProdMod = new float[100];
      this.ProdMod2 = new float[100];
      this.ProdMod3 = new float[100];
      this.ProdMod4 = new float[100];
      this.BattleForMod = new float[100];
      this.BattleVSMod = new float[100];
      if (hardcoded != 0)
        return;
      this.Name = "Universals".to_owned();
      this.SFIll = -1;
      this.SFExtra = -1;
      this.SidewaysFileName = "systemgraphics/trans.bmp";
      this.SidewaysFileName2 = "systemgraphics/trans.bmp";
      this.SidewaysFileName3 = "systemgraphics/trans.bmp";
      this.SidewaysFileName4 = "systemgraphics/trans.bmp";
      this.NationalFileName = "systemgraphics/trans.bmp";
      this.PeopleGroup = 0;
      let mut index: i32 =  0;
      do
      {
        this.BaseMorale[index] = 50;
        this.ProdMod[index] = 1f;
        this.BattleForMod[index] = 1f;
        this.BattleVSMod[index] = 1f;
        this.ProdMod2[index] = 1f;
        this.ProdMod3[index] = 1f;
        this.ProdMod4[index] = 1f;
        index += 1;
      }
      while (index <= 19);
      this.RegCol = -1;
      this.Red = -1;
      this.Green = -1;
      this.Blue = -1;
      this.BreakAt = 0;
      this.ExtraGraphicUse = -1;
      this.LibId = LibIdClass::new();
      this.tv0 = 0;
      this.tv1 = 0;
      this.tv2 = 0;
      this.artCode = 0;
    }

    pub fn Kill()
    {
      if (this.SidewaysSpriteID > 0)
      {
        BitmapStore.RemoveBitmapNr(this.SidewaysSpriteID);
        this.SidewaysSpriteID = -1;
      }
      if (this.SidewaysSpriteID2 > 0)
      {
        BitmapStore.RemoveBitmapNr(this.SidewaysSpriteID2);
        this.SidewaysSpriteID2 = -1;
      }
      if (this.SidewaysSpriteID3 > 0)
      {
        BitmapStore.RemoveBitmapNr(this.SidewaysSpriteID3);
        this.SidewaysSpriteID3 = -1;
      }
      if (this.SidewaysSpriteID4 > 0)
      {
        BitmapStore.RemoveBitmapNr(this.SidewaysSpriteID4);
        this.SidewaysSpriteID4 = -1;
      }
      if (this.NationalSpriteID <= 0)
        return;
      BitmapStore.RemoveBitmapNr(this.NationalSpriteID);
      this.NationalSpriteID = -1;
    }

    pub fn ReplaceSidewaysSprite(filename: String)
    {
      this.SidewaysFileName = filename;
      this.SidewaysSpriteID = BitmapStore.ReloadFile(this.SidewaysSpriteID, this.SidewaysFileName);
    }

    pub fn ReplaceSidewaysSprite2(filename: String)
    {
      this.SidewaysFileName2 = filename;
      this.SidewaysSpriteID2 = BitmapStore.ReloadFile(this.SidewaysSpriteID2, this.SidewaysFileName2);
    }

    pub fn ReplaceSidewaysSprite3(filename: String)
    {
      this.SidewaysFileName3 = filename;
      this.SidewaysSpriteID3 = BitmapStore.ReloadFile(this.SidewaysSpriteID3, this.SidewaysFileName3);
    }

    pub fn ReplaceSidewaysSprite4(filename: String)
    {
      this.SidewaysFileName4 = filename;
      this.SidewaysSpriteID4 = BitmapStore.ReloadFile(this.SidewaysSpriteID4, this.SidewaysFileName4);
    }

    pub fn ReplaceNationalSprite(filename: String)
    {
      this.NationalFileName = filename;
      this.NationalSpriteID = BitmapStore.ReloadFile(this.NationalSpriteID, this.NationalFileName, IsBig: true);
    }

    pub fn LoadSprites()
    {
      this.SidewaysSpriteID = BitmapStore.AddFile(this.SidewaysFileName, false);
      this.NationalSpriteID = BitmapStore.AddFile(this.NationalFileName, false, true);
      this.SidewaysSpriteID2 = BitmapStore.AddFile(this.SidewaysFileName2, false);
      this.SidewaysSpriteID3 = BitmapStore.AddFile(this.SidewaysFileName3, false);
      this.SidewaysSpriteID4 = BitmapStore.AddFile(this.SidewaysFileName4, false);
    }
  }
}
