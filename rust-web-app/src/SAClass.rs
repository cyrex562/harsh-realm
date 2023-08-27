// Decompiled with JetBrains decompiler
// Type: WindowsApplication1.SAClass
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
  pub class SAClass : ISerializable
  {
    pub X: i32;
    pub Y: i32;
    pub Size: i32;
    pub aivp: i32;
    pub fuzzyvp: i32;
    pub Neighbour: Vec<i32>;
    pub NeighbourCount: i32;
    pub SeaNeighbour: Vec<i32>;
    pub SeaNeighbourCount: i32;
    pub Constitutant: Vec<i32>;
    pub ConstitutantCount: i32;
    pub LandReservePlan: i32;

    pub SAClass()
    {
      self.Neighbour = new int[1];
      self.SeaNeighbour = new int[1];
      self.Constitutant = new int[1];
    }

    pub SAClass Clone()
    {
      BinaryFormatter binaryFormatter = BinaryFormatter::new();
      MemoryStream serializationStream = MemoryStream::new();
      binaryFormatter.Serialize((Stream) serializationStream,  this);
      serializationStream.Position = 0L;
      return (SAClass) binaryFormatter.Deserialize((Stream) serializationStream);
    }

    pub fn AddNeighbour(nr: i32)
    {
      let mut neighbourCount: i32 = self.NeighbourCount;
      for (let mut index: i32 = 1; index <= neighbourCount; index += 1)
      {
        if (self.Neighbour[index] == nr)
          return;
      }
      this += 1.NeighbourCount;
      self.Neighbour = (int[]) Utils.CopyArray((Array) self.Neighbour, (Array) new int[self.NeighbourCount + 1]);
      self.Neighbour[self.NeighbourCount] = nr;
    }

    pub IsNeighbour: bool(nr: i32)
    {
      if (self.NeighbourCount <= 0)
        return false;
      let mut neighbourCount: i32 = self.NeighbourCount;
      for (let mut index: i32 = 1; index <= neighbourCount; index += 1)
      {
        if (self.Neighbour[index] == nr)
          return true;
      }
      return false;
    }

    pub fn AddSeaNeighbour(nr: i32)
    {
      let mut seaNeighbourCount: i32 = self.SeaNeighbourCount;
      for (let mut index: i32 = 1; index <= seaNeighbourCount; index += 1)
      {
        if (self.SeaNeighbour[index] == nr)
          return;
      }
      this += 1.SeaNeighbourCount;
      self.SeaNeighbour = (int[]) Utils.CopyArray((Array) self.SeaNeighbour, (Array) new int[self.SeaNeighbourCount + 1]);
      self.SeaNeighbour[self.SeaNeighbourCount] = nr;
    }

    pub IsSeaNeighbour: bool(nr: i32)
    {
      if (self.SeaNeighbourCount <= 0)
        return false;
      let mut seaNeighbourCount: i32 = self.SeaNeighbourCount;
      for (let mut index: i32 = 1; index <= seaNeighbourCount; index += 1)
      {
        if (self.SeaNeighbour[index] == nr)
          return true;
      }
      return false;
    }

    pub fn AddConstitutant(nr: i32)
    {
      let mut constitutantCount: i32 = self.ConstitutantCount;
      for (let mut index: i32 = 1; index <= constitutantCount; index += 1)
      {
        if (self.Constitutant[index] == nr)
          return;
      }
      this += 1.ConstitutantCount;
      self.Constitutant = (int[]) Utils.CopyArray((Array) self.Constitutant, (Array) new int[self.ConstitutantCount + 1]);
      self.Constitutant[self.ConstitutantCount] = nr;
    }

    pub IsConstitutant: bool(nr: i32)
    {
      if (self.ConstitutantCount <= 0)
        return false;
      let mut constitutantCount: i32 = self.ConstitutantCount;
      for (let mut index: i32 = 1; index <= constitutantCount; index += 1)
      {
        if (self.Constitutant[index] == nr)
          return true;
      }
      return false;
    }

    pub fn GetObjectData(SerializationInfo info, StreamingContext context)
    {
      info.AddValue("X", self.X);
      info.AddValue("Y", self.Y);
      info.AddValue("Size", self.Size);
      info.AddValue("aivp", self.aivp);
      info.AddValue("fuzzyvp", self.fuzzyvp);
      info.AddValue("Neighbour",  self.Neighbour);
      info.AddValue("NeighbourCount", self.NeighbourCount);
      info.AddValue("SeaNeighbour",  self.SeaNeighbour);
      info.AddValue("SeaNeighbourCount", self.SeaNeighbourCount);
      info.AddValue("Constitutant",  self.Constitutant);
      info.AddValue("ConstitutantCount", self.ConstitutantCount);
      info.AddValue("LandReservePlan", self.LandReservePlan);
    }

    protected SAClass(SerializationInfo info, StreamingContext context)
    {
      self.Neighbour = new int[1];
      self.SeaNeighbour = new int[1];
      self.Constitutant = new int[1];
      self.X = info.GetInt32(nameof (X));
      self.Y = info.GetInt32(nameof (Y));
      self.Size = info.GetInt32(nameof (Size));
      self.aivp = info.GetInt32(nameof (aivp));
      self.fuzzyvp = info.GetInt32(nameof (fuzzyvp));
      self.NeighbourCount = info.GetInt32(nameof (NeighbourCount));
      self.SeaNeighbourCount = info.GetInt32(nameof (SeaNeighbourCount));
      self.ConstitutantCount = info.GetInt32(nameof (ConstitutantCount));
      self.Neighbour = new int[self.NeighbourCount + 1];
      self.SeaNeighbour = new int[self.SeaNeighbourCount + 1];
      self.Constitutant = new int[self.ConstitutantCount + 1];
      self.Neighbour = (int[]) info.GetValue(nameof (Neighbour), self.Neighbour.GetType());
      self.Constitutant = (int[]) info.GetValue(nameof (Constitutant), self.Constitutant.GetType());
      self.LandReservePlan = info.GetInt32(nameof (LandReservePlan));
    }
  }
}
