// Decompiled with JetBrains decompiler
// Type: WindowsApplication1.MasterLogClass
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
  pub class MasterLogClass : ISerializable
  {
    pub Counter: i32;
    pub DateTime LastSave;
    pub Code: Vec<i32>;
    pub Round: Vec<i32>;
    pub Qty: Vec<i32>;
    pub Turn: Vec<i32>;
    pub RandomCode: Vec<i32>;

    pub fn GetObjectData(SerializationInfo info, StreamingContext context)
    {
      info.AddValue("Counter", this.Counter);
      info.AddValue("Code",  this.Code);
      info.AddValue("Round",  this.Round);
      info.AddValue("Qty",  this.Qty);
      info.AddValue("Turn",  this.Turn);
      info.AddValue("LastSave", this.LastSave);
      info.AddValue("RandomCode",  this.RandomCode);
    }

    protected MasterLogClass(SerializationInfo info, StreamingContext context)
    {
      this.Code = new int[1];
      this.Round = new int[1];
      this.Qty = new int[1];
      this.Turn = new int[1];
      this.RandomCode = new int[1];
      this.Counter = info.GetInt32(nameof (Counter));
      this.Code = (int[]) info.GetValue(nameof (Code), this.Code.GetType());
      this.Round = (int[]) info.GetValue(nameof (Round), this.Round.GetType());
      this.Qty = (int[]) info.GetValue(nameof (Qty), this.Qty.GetType());
      this.Turn = (int[]) info.GetValue(nameof (Turn), this.Turn.GetType());
      this.LastSave = info.GetDateTime(nameof (LastSave));
      try
      {
        this.RandomCode = (int[]) info.GetValue(nameof (RandomCode), this.RandomCode.GetType());
      }
      catch (Exception ex)
      {
        ProjectData.SetProjectError(ex);
        this.RandomCode = new int[this.Counter + 1];
        let mut counter: i32 =  this.Counter;
        for (let mut index: i32 =  0; index <= counter; index += 1)
          this.RandomCode[index] = 0;
        ProjectData.ClearProjectError();
      }
    }

    pub fn serialize(fileloc: String)
    {
      FileStream serializationStream = new FileStream(fileloc, FileMode.Create, FileAccess.Write, FileShare.ReadWrite);
      BinaryFormatter::new().Serialize((Stream) serializationStream,  this);
      serializationStream.Close();
    }

    pub static MasterLogClass deserialize(fileloc: String)
    {
      FileStream serializationStream = new FileStream(fileloc, FileMode.Open, FileAccess.Read, FileShare.ReadWrite);
      MasterLogClass masterLogClass = (MasterLogClass) BinaryFormatter::new().Deserialize((Stream) serializationStream);
      serializationStream.Flush();
      serializationStream.Close();
      return masterLogClass;
    }

    pub fn SetEntry(gameid: i32, roundnr: i32, turnnr: i32, rancode: i32)
    {
      let mut index1: i32 =  -1;
      let mut counter: i32 =  this.Counter;
      for (let mut index2: i32 =  0; index2 <= counter; index2 += 1)
      {
        if (this.Code[index2] == gameid & this.Round[index2] == roundnr & this.Turn[index2] == turnnr)
        {
          index1 = index2;
          break;
        }
      }
      if (index1 == -1)
      {
        this += 1.Counter;
        this.Code = (int[]) Utils.CopyArray((Array) this.Code, (Array) new int[this.Counter + 1]);
        this.Round = (int[]) Utils.CopyArray((Array) this.Round, (Array) new int[this.Counter + 1]);
        this.Qty = (int[]) Utils.CopyArray((Array) this.Qty, (Array) new int[this.Counter + 1]);
        this.Turn = (int[]) Utils.CopyArray((Array) this.Turn, (Array) new int[this.Counter + 1]);
        this.RandomCode = (int[]) Utils.CopyArray((Array) this.RandomCode, (Array) new int[this.Counter + 1]);
        index1 = this.Counter;
      }
      this.Code[index1] = gameid;
      this.Round[index1] = roundnr;
      int[] qty = this.Qty;
      int[] numArray = qty;
      let mut index3: i32 =  index1;
      let mut index4: i32 =  index3;
      let mut num: i32 =  qty[index3] + 1;
      numArray[index4] = num;
      this.Turn[index1] = turnnr;
      this.RandomCode[index1] = rancode;
    }

    pub object ReturnQty(gameid: i32, roundnr: i32, turnnr: i32, rancode: i32)
    {
      let mut index1: i32 =  -1;
      let mut counter: i32 =  this.Counter;
      for (let mut index2: i32 =  0; index2 <= counter; index2 += 1)
      {
        if (this.Code[index2] == gameid & this.Round[index2] == roundnr & this.Turn[index2] == turnnr & (rancode == -1 | this.RandomCode[index2] == rancode | this.RandomCode[index2] == 0))
        {
          index1 = index2;
          break;
        }
      }
      return index1 > -1 ?  this.Qty[index1] :  0;
    }

    pub MasterLogClass()
    {
      this.Code = new int[1];
      this.Round = new int[1];
      this.Qty = new int[1];
      this.Turn = new int[1];
      this.RandomCode = new int[1];
      this.Counter = -1;
    }
  }
}
