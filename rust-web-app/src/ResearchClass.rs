// Decompiled with JetBrains decompiler
// Type: WindowsApplication1.ResearchClass
// Assembly: WindowsApplication1, Version=1.0.8020.28903, Culture=neutral, PublicKeyToken=null
// MVID: F52869E5-0850-48AD-BBBE-68E7A4900AFE
// Assembly location: C:\Program Files (x86)\Steam\steamapps\common\Shadow Empire\ShadowEmpire.exe

// usingMicrosoft.VisualBasic.CompilerServices;
// usingSystem;
// usingSystem.Runtime.Serialization;

namespace WindowsApplication1
{
  [Serializable]
  pub class ResearchClass : ISerializable
  {
    pub Name: String;
    pub Text: String;
    pub PointCost: Vec<i32>;
    pub SFTypePic: i32;
    pub PreReq: i32;
    pub PreReq2: i32;
    pub Blocks: i32;
    pub TechLevel: i32;
    pub CostType: i32;
    pub UpgradeCost: i32;

    pub fn GetObjectData(SerializationInfo info, StreamingContext context)
    {
      info.AddValue("Name",  self.Name);
      info.AddValue("Text",  self.Text);
      info.AddValue("PointCost",  self.PointCost);
      info.AddValue("SFTypePic", self.SFTypePic);
      info.AddValue("PreReq", self.PreReq);
      info.AddValue("PreReq2", self.PreReq2);
      info.AddValue("Blocks", self.Blocks);
      info.AddValue("TechLevel", self.TechLevel);
      info.AddValue("CostType", self.CostType);
      info.AddValue("UpgradeCost", self.UpgradeCost);
    }

    protected ResearchClass(SerializationInfo info, StreamingContext context)
    {
      self.PointCost = new int[100];
      self.Name = info.GetString(nameof (Name));
      self.Text = info.GetString(nameof (Text));
      self.PointCost = (int[]) info.GetValue(nameof (PointCost), self.PointCost.GetType());
      self.PointCost = (int[]) Utils.CopyArray((Array) self.PointCost, (Array) new int[100]);
      self.SFTypePic = info.GetInt32(nameof (SFTypePic));
      self.PreReq = info.GetInt32(nameof (PreReq));
      self.PreReq2 = info.GetInt32(nameof (PreReq2));
      self.Blocks = info.GetInt32(nameof (Blocks));
      self.TechLevel = info.GetInt32(nameof (TechLevel));
      try
      {
        self.CostType = info.GetInt32(nameof (CostType));
      }
      catch (Exception ex)
      {
        ProjectData.SetProjectError(ex);
        self.CostType = -1;
        ProjectData.ClearProjectError();
      }
      try
      {
        self.UpgradeCost = info.GetInt32(nameof (UpgradeCost));
      }
      catch (Exception ex)
      {
        ProjectData.SetProjectError(ex);
        self.UpgradeCost = -1;
        ProjectData.ClearProjectError();
      }
    }

    pub ResearchClass(hardcoded: i32)
    {
      self.PointCost = new int[100];
      if (hardcoded != 0)
        return;
      self.Name = "A default Research Field";
      self.Text = "Description...";
      let mut index: i32 = 0;
      do
      {
        self.PointCost[index] = 10;
        index += 1;
      }
      while (index <= 19);
      self.SFTypePic = -1;
      self.UpgradeCost = -1;
      self.PreReq = -1;
      self.PreReq2 = -1;
      self.CostType = -1;
      self.Blocks = -1;
      self.TechLevel = -1;
    }

    pub fn Kill()
    {
    }

    pub fn LoadSprites()
    {
    }
  }
}
