// Decompiled with JetBrains decompiler
// Type: WindowsApplication1.CommandClass
// Assembly: WindowsApplication1, Version=1.0.8020.28903, Culture=neutral, PublicKeyToken=null
// MVID: F52869E5-0850-48AD-BBBE-68E7A4900AFE
// Assembly location: C:\Program Files (x86)\Steam\steamapps\common\Shadow Empire\ShadowEmpire.exe

// usingMicrosoft.VisualBasic;
// usingMicrosoft.VisualBasic.CompilerServices;
// usingSystem;
// usingSystem.IO;
// usingSystem.Runtime.Serialization;
// usingSystem.Runtime.Serialization.Formatters.Binary;

namespace WindowsApplication1
{
  [Serializable]
  pub class CommandClass : ISerializable
  {
    pub type: i32;
    pub string[,] Data;
    pub DataString: String;

    pub fn GetObjectData(SerializationInfo info, StreamingContext context)
    {
      info.AddValue("Type", this.type);
      if (DrawMod.TGame.Data.Version > 407)
      {
        let mut index1: i32 =  0;
        str: String;
        do
        {
          let mut index2: i32 =  0;
          do
          {
            str = str + "®" + this.Data[index1, index2];
            index2 += 1;
          }
          while (index2 <= 30);
          index1 += 1;
        }
        while (index1 <= 2);
        info.AddValue("s",  str);
      }
      else
        info.AddValue("Data",  this.Data);
      info.AddValue("DataString",  this.DataString);
    }

    protected CommandClass(SerializationInfo info, StreamingContext context)
    {
      this.Data = new string[3, 31];
      numArray1: Vec<i32> = new int[3, 31];
      this.type = info.GetInt32("Type");
      try
      {
        if (DrawMod.TGame.Data.Version > 407)
        {
          str1: String = info.GetString("s");
          let mut Start1: i32 =  1;
          let mut index1: i32 =  0;
          do
          {
            let mut index2: i32 =  0;
            do
            {
              let mut num: i32 =  Strings.InStr(Start1, str1, "®", CompareMethod.Text);
              let mut Start2: i32 =  Strings.InStr(num + 1, str1, "®", CompareMethod.Text);
              str2: String = "";
              if (num > 0)
              {
                if (Start2 > 0)
                {
                  str2 = Strings.Mid(str1, num + 1, Start2 - num - 1);
                  str1 = Strings.Mid(str1, Start2);
                }
                else
                {
                  str2 = Strings.Mid(str1, num + 1);
                  str1 = "";
                }
              }
              this.Data[index1, index2] = str2;
              index2 += 1;
            }
            while (index2 <= 30);
            index1 += 1;
          }
          while (index1 <= 2);
        }
        else
        {
          try
          {
            this.Data = (string[,]) info.GetValue(nameof (Data), this.Data.GetType());
          }
          catch (Exception ex)
          {
            ProjectData.SetProjectError(ex);
            numArray1 = (int[,]) info.GetValue(nameof (Data), numArray1.GetType());
            let mut index3: i32 =  0;
            do
            {
              let mut index4: i32 =  0;
              do
              {
                this.Data[index3, index4] = Strings.Trim(Conversion.Str( numArray1[index3, index4]));
                index4 += 1;
              }
              while (index4 <= 30);
              index3 += 1;
            }
            while (index3 <= 2);
            ProjectData.ClearProjectError();
          }
        }
      }
      catch (Exception ex1)
      {
        ProjectData.SetProjectError(ex1);
        try
        {
          this.Data = (string[,]) info.GetValue(nameof (Data), this.Data.GetType());
        }
        catch (Exception ex2)
        {
          ProjectData.SetProjectError(ex2);
          numArray2: Vec<i32> = (int[,]) info.GetValue(nameof (Data), numArray1.GetType());
          let mut index5: i32 =  0;
          do
          {
            let mut index6: i32 =  0;
            do
            {
              this.Data[index5, index6] = Strings.Trim(Conversion.Str( numArray2[index5, index6]));
              index6 += 1;
            }
            while (index6 <= 30);
            index5 += 1;
          }
          while (index5 <= 2);
          ProjectData.ClearProjectError();
        }
        ProjectData.ClearProjectError();
      }
      this.DataString = info.GetString(nameof (DataString));
    }

    pub CommandClass Clone()
    {
      BinaryFormatter binaryFormatter = BinaryFormatter::new();
      MemoryStream serializationStream = MemoryStream::new();
      binaryFormatter.Serialize((Stream) serializationStream,  this);
      serializationStream.Position = 0L;
      return (CommandClass) binaryFormatter.Deserialize((Stream) serializationStream);
    }

    pub CommandClass(hardcoded: i32)
    {
      this.Data = new string[3, 31];
      this.type = 0;
    }
  }
}
