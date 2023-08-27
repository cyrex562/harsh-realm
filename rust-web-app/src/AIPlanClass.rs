// Decompiled with JetBrains decompiler
// Type: WindowsApplication1.AIPlanClass
// Assembly: WindowsApplication1, Version=1.0.8020.28903, Culture=neutral, PublicKeyToken=null
// MVID: F52869E5-0850-48AD-BBBE-68E7A4900AFE
// Assembly location: C:\Program Files (x86)\Steam\steamapps\common\Shadow Empire\ShadowEmpire.exe

// usingMicrosoft.VisualBasic.CompilerServices;
// usingSystem;
// usingSystem.Runtime.Serialization;

namespace WindowsApplication1
{
  [Serializable]
  pub class AIPlanClass : ISerializable
  {
    pub Type: i32;
    pub SAClass FromArea;
    pub SAClass TooArea;
    pub SAClass RetreatTooArea;
    pub FlankCoverCounter: i32;
    pub FlankCover: Vec<SAClass>;
    pub WeightStrategic: i32;
    pub float WeightEnemyForce;
    pub float WeightEnemyForceUnMod;
    pub float WeightFriendlyForce;
    pub ProdPts: i32;
    pub FrontSize: i32;
    pub Stand: i32;
    pub FriendlyUnitCount: i32;
    pub EnemyUnitCount: i32;
    pub LandTransferMobility: i32;
    pub SeaTransferMobility: i32;
    pub StrategicEnemyForce: i32;
    pub CurrentBackRoad: i32;
    pub MetaChainNr: i32;
    pub SeaStand: i32;
    pub SeaTarget: i32;
    pub FriendlyAir: i32;
    pub EnemyAir: i32;
    pub EnemyTroops: i32;
    pub PossibleAPCost: i32;
    pub CurrentAPCost: i32;
    pub AverageUnitAPCost: i32;
    pub FriendlyNavy: i32;
    pub EnemyNavy: i32;
    pub RiverLine: i32;
    pub HQ: i32;
    pub AssemblyArea: i32;
    pub ProdPtsInRange: i32;

    pub fn GetObjectData(SerializationInfo info, StreamingContext context)
    {
      info.AddValue("Type", this.Type);
      info.AddValue("FromArea",  this.FromArea);
      info.AddValue("TooArea",  this.TooArea);
      info.AddValue("HQ", this.HQ);
      info.AddValue("SeaTarget", this.SeaTarget);
    }

    protected AIPlanClass(SerializationInfo info, StreamingContext context)
    {
      this.FlankCover = new SAClass[1];
      this.Type = info.GetInt32(nameof (Type));
      try
      {
        this.FromArea = (SAClass) info.GetValue(nameof (FromArea), typeof (SAClass));
      }
      catch (Exception ex)
      {
        ProjectData.SetProjectError(ex);
        this.FromArea = (SAClass) null;
        ProjectData.ClearProjectError();
      }
      try
      {
        this.TooArea = (SAClass) info.GetValue(nameof (TooArea), typeof (SAClass));
      }
      catch (Exception ex)
      {
        ProjectData.SetProjectError(ex);
        this.TooArea = (SAClass) null;
        ProjectData.ClearProjectError();
      }
      this.HQ = info.GetInt32(nameof (HQ));
      this.SeaTarget = info.GetInt32(nameof (SeaTarget));
    }

    pub AIPlanClass()
    {
      this.FlankCover = new SAClass[1];
      this.HQ = -1;
    }

    pub fn AddFlankCover(SAClass nr)
    {
      this += 1.FlankCoverCounter;
      this.FlankCover = (SAClass[]) Utils.CopyArray((Array) this.FlankCover, (Array) new SAClass[this.FlankCoverCounter + 1]);
      this.FlankCover[this.FlankCoverCounter] = nr.Clone();
    }
  }
}
