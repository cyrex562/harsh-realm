// Decompiled with JetBrains decompiler
// Type: WindowsApplication1.SFClass
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
  pub class SFClass : ISerializable
  {
    pub Type: i32;
    pub Qty: i32;
    pub Ap: i32;
    pub Rdn: i32;
    pub Xp: i32;
    pub People: i32;
    pub CurrentEntrench: i32;
    pub Mor: i32;
    pub EP: i32;
    pub OffMod: i32;
    pub DefMod: i32;
    pub MoveType: i32;
    pub initialEntrench: i32;
    pub Vigor: i32;

    pub SFClass Clone()
    {
      BinaryFormatter binaryFormatter = BinaryFormatter::new();
      MemoryStream serializationStream = MemoryStream::new();
      binaryFormatter.Serialize((Stream) serializationStream,  this);
      serializationStream.Position = 0L;
      return (SFClass) binaryFormatter.Deserialize((Stream) serializationStream);
    }

    pub fn GetObjectData(SerializationInfo info, StreamingContext context)
    {
      info.AddValue("Type", self.Type);
      info.AddValue("Qty", self.Qty);
      info.AddValue("Ap", self.Ap);
      info.AddValue("Rdn", self.Rdn);
      info.AddValue("Xp", self.Xp);
      info.AddValue("People", self.People);
      info.AddValue("CurrentEntrench", self.CurrentEntrench);
      info.AddValue("Mor", self.Mor);
      info.AddValue("EP", self.EP);
      info.AddValue("OffMod", self.OffMod);
      info.AddValue("DefMod", self.DefMod);
      info.AddValue("MoveType", self.MoveType);
      info.AddValue("initialEntrench", self.initialEntrench);
      info.AddValue("Vigor", self.Vigor);
    }

    protected SFClass(SerializationInfo info, StreamingContext context)
    {
      self.Type = info.GetInt32(nameof (Type));
      self.Qty = info.GetInt32(nameof (Qty));
      self.Ap = info.GetInt32(nameof (Ap));
      self.Rdn = info.GetInt32(nameof (Rdn));
      self.Xp = info.GetInt32(nameof (Xp));
      self.People = info.GetInt32(nameof (People));
      self.CurrentEntrench = info.GetInt32(nameof (CurrentEntrench));
      self.Mor = info.GetInt32(nameof (Mor));
      self.EP = info.GetInt32(nameof (EP));
      if (DrawMod.TGame.Data.Version < 158)
      {
        if (DrawMod.TGame.Data.Version >= 110)
        {
          self.OffMod = info.GetInt32(nameof (OffMod));
          self.DefMod = info.GetInt32(nameof (DefMod));
        }
        else
        {
          self.OffMod = 0;
          self.DefMod = 0;
        }
        if (DrawMod.TGame.Data.Version < 130)
        {
          self.MoveType = -1;
        }
        else
        {
          try
          {
            self.MoveType = info.GetInt32(nameof (MoveType));
          }
          catch (Exception ex)
          {
            ProjectData.SetProjectError(ex);
            self.MoveType = -1;
            ProjectData.ClearProjectError();
          }
        }
      }
      else
      {
        self.OffMod = info.GetInt32(nameof (OffMod));
        self.DefMod = info.GetInt32(nameof (DefMod));
        self.MoveType = info.GetInt32(nameof (MoveType));
      }
      try
      {
        self.initialEntrench = info.GetInt32(nameof (initialEntrench));
      }
      catch (Exception ex)
      {
        ProjectData.SetProjectError(ex);
        self.initialEntrench = self.CurrentEntrench;
        ProjectData.ClearProjectError();
      }
      try
      {
        self.Vigor = info.GetInt32(nameof (Vigor));
      }
      catch (Exception ex)
      {
        ProjectData.SetProjectError(ex);
        self.Vigor = 100;
        ProjectData.ClearProjectError();
      }
    }

    pub SFClass(hardcoded: i32)
    {
      self.Type = -1;
      self.MoveType = -1;
      self.initialEntrench = 0;
      self.Vigor = 100;
    }

    pub fn Kill()
    {
    }

    pub fn LoadSprites()
    {
    }
  }
}
