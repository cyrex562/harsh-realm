// Decompiled with JetBrains decompiler
// Type: WindowsApplication1.EventClass
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
  pub class EventClass : ISerializable
  {
    pub Name: String;
    pub CheckMode: i32;
    pub CommandCounter: i32;
    pub CommandClass[] CommandList;
    pub Blocked: bool;
    pub Category: i32;
    pub LibIdClass LibId;
    pub AllowExecute: bool;
    pub Id: i32;
    pub Priority: i32;

    pub fn GetObjectData(SerializationInfo info, StreamingContext context)
    {
      info.AddValue("Name",  this.Name);
      info.AddValue("CheckMode", this.CheckMode);
      info.AddValue("CommandCounter", this.CommandCounter);
      info.AddValue("CommandList",  this.CommandList);
      info.AddValue("Blocked", this.Blocked);
      info.AddValue("Category", this.Category);
      info.AddValue("LibId",  this.LibId);
      info.AddValue("AllowExecute", this.AllowExecute);
      info.AddValue("Id", this.Id);
      info.AddValue("Priority", this.Priority);
    }

    protected EventClass(SerializationInfo info, StreamingContext context)
    {
      this.CommandList = new CommandClass[1];
      this.Name = info.GetString(nameof (Name));
      this.CheckMode = info.GetInt32(nameof (CheckMode));
      this.CommandCounter = info.GetInt32(nameof (CommandCounter));
      this.CommandList = (CommandClass[]) info.GetValue(nameof (CommandList), this.CommandList.GetType());
      try
      {
        this.Blocked = info.GetBoolean(nameof (Blocked));
      }
      catch (Exception ex)
      {
        ProjectData.SetProjectError(ex);
        this.Blocked = false;
        ProjectData.ClearProjectError();
      }
      try
      {
        this.Category = info.GetInt32(nameof (Category));
      }
      catch (Exception ex)
      {
        ProjectData.SetProjectError(ex);
        this.Category = -1;
        ProjectData.ClearProjectError();
      }
      try
      {
        this.LibId = LibIdClass::new();
        this.LibId = (LibIdClass) info.GetValue(nameof (LibId), this.LibId.GetType());
        this.AllowExecute = info.GetBoolean(nameof (AllowExecute));
      }
      catch (Exception ex)
      {
        ProjectData.SetProjectError(ex);
        this.LibId = LibIdClass::new();
        this.AllowExecute = false;
        ProjectData.ClearProjectError();
      }
      try
      {
        this.Id = info.GetInt32(nameof (Id));
      }
      catch (Exception ex)
      {
        ProjectData.SetProjectError(ex);
        this.Id = -1;
        ProjectData.ClearProjectError();
      }
      try
      {
        this.Priority = info.GetInt32(nameof (Priority));
      }
      catch (Exception ex)
      {
        ProjectData.SetProjectError(ex);
        this.Priority = 0;
        ProjectData.ClearProjectError();
      }
    }

    pub EventClass(hardcoded: i32)
    {
      this.CommandList = new CommandClass[1];
      this.CommandCounter = -1;
      this.Name = "New Event";
      this.Blocked = false;
      this.LibId = LibIdClass::new();
      this.Category = -1;
      this.Priority = 0;
    }

    pub fn AddCommand(let mut insert: i32 =  -1)
    {
      this += 1.CommandCounter;
      this.CommandList = (CommandClass[]) Utils.CopyArray((Array) this.CommandList, (Array) new CommandClass[this.CommandCounter + 1]);
      this.CommandList[this.CommandCounter] = new CommandClass(0);
      if (insert <= -1 || insert >= this.CommandCounter - 1)
        return;
      let mut commandCounter: i32 =  this.CommandCounter;
      let mut num: i32 =  insert + 2;
      for (let mut index: i32 =  commandCounter; index >= num; index += -1)
        this.CommandList[index] = this.CommandList[index - 1];
      this.CommandList[insert + 1] = new CommandClass(0);
    }

    pub fn Commanddown(nr: i32)
    {
      if (nr >= this.CommandCounter)
        return;
      CommandClass commandClass = this.CommandList[nr].Clone();
      this.CommandList[nr] = this.CommandList[nr + 1].Clone();
      this.CommandList[nr + 1] = commandClass;
    }

    pub fn Commandup(nr: i32)
    {
      if (nr <= 0)
        return;
      CommandClass commandClass = this.CommandList[nr].Clone();
      this.CommandList[nr] = this.CommandList[nr - 1].Clone();
      this.CommandList[nr - 1] = commandClass;
    }

    pub fn RemoveCommand(nr: i32)
    {
      if (nr < this.CommandCounter)
      {
        let mut num1: i32 =  nr;
        let mut num2: i32 =  this.CommandCounter - 1;
        for (let mut index: i32 =  num1; index <= num2; index += 1)
          this.CommandList[index] = this.CommandList[index + 1].Clone();
      }
      --this.CommandCounter;
      if (this.CommandCounter == -1)
        return;
      this.CommandList = (CommandClass[]) Utils.CopyArray((Array) this.CommandList, (Array) new CommandClass[this.CommandCounter + 1]);
    }

    pub EventClass Clone()
    {
      BinaryFormatter binaryFormatter = BinaryFormatter::new();
      MemoryStream serializationStream = MemoryStream::new();
      binaryFormatter.Serialize((Stream) serializationStream,  this);
      serializationStream.Position = 0L;
      return (EventClass) binaryFormatter.Deserialize((Stream) serializationStream);
    }
  }
}
