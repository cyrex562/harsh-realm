// Decompiled with JetBrains decompiler
// Type: WindowsApplication1.HistoryStepClass
// Assembly: WindowsApplication1, Version=1.0.8020.28903, Culture=neutral, PublicKeyToken=null
// MVID: F52869E5-0850-48AD-BBBE-68E7A4900AFE
// Assembly location: C:\Program Files (x86)\Steam\steamapps\common\Shadow Empire\ShadowEmpire.exe

// usingMicrosoft.VisualBasic.CompilerServices;
// usingSystem;
// usingSystem.Runtime.Serialization;

namespace WindowsApplication1
{
  [Serializable]
  pub class HistoryStepClass : ISerializable
  {
    pub Regime: i32;
    pub X: i32;
    pub Y: i32;
    pub Map: i32;
    pub Ownership: i32;
    pub Force: i32;
    pub SFType: i32;
    pub AttackDirection: Vec<i32>;
    pub AttackOtherType: i32;
    pub StepNr: i32;
    pub InfoString: String;
    pub His: i32;
    pub Depth: i32;
    pub LossCounter: i32;
    pub LossSFType: Vec<i32>;
    pub LossAttacker: Vec<i32>;
    pub LossOK: Vec<i32>;
    pub LossDEAD: Vec<i32>;
    pub LossRegimeWin: i32;
    pub LossAttReg: i32;
    pub LossDefReg: i32;

    pub fn GetObjectData(SerializationInfo info, StreamingContext context)
    {
      info.AddValue("Regime", this.Regime);
      info.AddValue("X", this.X);
      info.AddValue("Y", this.Y);
      info.AddValue("Map", this.Map);
      info.AddValue("Ownership", this.Ownership);
      info.AddValue("Force", this.Force);
      info.AddValue("SFType", this.SFType);
      info.AddValue("AttackDirection",  this.AttackDirection);
      info.AddValue("AttackOtherType", this.AttackOtherType);
      info.AddValue("StepNr", this.StepNr);
      info.AddValue("LossCounter", this.LossCounter);
      info.AddValue("LossSFType",  this.LossSFType);
      info.AddValue("LossOK",  this.LossOK);
      info.AddValue("LossDEAD",  this.LossDEAD);
      info.AddValue("LossAttacker",  this.LossAttacker);
      info.AddValue("LossRegimeWin", this.LossRegimeWin);
      info.AddValue("LossDefReg", this.LossDefReg);
      info.AddValue("LossAttReg", this.LossAttReg);
      info.AddValue("InfoString",  this.InfoString);
      info.AddValue("His", this.His);
      info.AddValue("Depth", this.Depth);
    }

    protected HistoryStepClass(SerializationInfo info, StreamingContext context)
    {
      this.AttackDirection = new int[6];
      this.LossSFType = new int[1];
      this.LossAttacker = new int[1];
      this.LossOK = new int[1];
      this.LossDEAD = new int[1];
      this.Regime = info.GetInt32(nameof (Regime));
      this.X = info.GetInt32(nameof (X));
      this.Y = info.GetInt32(nameof (Y));
      try
      {
        this.Map = info.GetInt32(nameof (Map));
      }
      catch (Exception ex)
      {
        ProjectData.SetProjectError(ex);
        this.Map = 0;
        ProjectData.ClearProjectError();
      }
      this.Ownership = info.GetInt32(nameof (Ownership));
      this.Force = info.GetInt32(nameof (Force));
      this.SFType = info.GetInt32(nameof (SFType));
      this.AttackDirection = (int[]) info.GetValue(nameof (AttackDirection), this.AttackDirection.GetType());
      this.AttackOtherType = info.GetInt32(nameof (AttackOtherType));
      this.StepNr = info.GetInt32(nameof (StepNr));
      this.LossCounter = info.GetInt32(nameof (LossCounter));
      if (this.LossCounter > -1)
      {
        this.LossSFType = new int[this.LossCounter + 1];
        this.LossAttacker = new int[this.LossCounter + 1];
        this.LossOK = new int[this.LossCounter + 1];
        this.LossDEAD = new int[this.LossCounter + 1];
      }
      else
      {
        this.LossSFType = new int[1];
        this.LossAttacker = new int[1];
        this.LossOK = new int[1];
        this.LossDEAD = new int[1];
      }
      this.LossSFType = (int[]) info.GetValue(nameof (LossSFType), this.LossSFType.GetType());
      this.LossAttacker = (int[]) info.GetValue(nameof (LossAttacker), this.LossAttacker.GetType());
      this.LossOK = (int[]) info.GetValue(nameof (LossOK), this.LossOK.GetType());
      this.LossDEAD = (int[]) info.GetValue(nameof (LossDEAD), this.LossDEAD.GetType());
      this.LossRegimeWin = info.GetInt32(nameof (LossRegimeWin));
      this.LossDefReg = info.GetInt32(nameof (LossDefReg));
      this.LossAttReg = info.GetInt32(nameof (LossAttReg));
      this.InfoString = info.GetString(nameof (InfoString));
      try
      {
        this.His = info.GetInt32(nameof (His));
        this.Depth = info.GetInt32(nameof (Depth));
      }
      catch (Exception ex)
      {
        ProjectData.SetProjectError(ex);
        this.His = -1;
        this.Depth = -1;
        ProjectData.ClearProjectError();
      }
    }

    pub HistoryStepClass(hardcoded: i32)
    {
      this.AttackDirection = new int[6];
      this.LossSFType = new int[1];
      this.LossAttacker = new int[1];
      this.LossOK = new int[1];
      this.LossDEAD = new int[1];
      this.AttackOtherType = -1;
    }
  }
}
