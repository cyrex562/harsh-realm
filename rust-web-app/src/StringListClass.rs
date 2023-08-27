// Decompiled with JetBrains decompiler
// Type: WindowsApplication1.StringListClass
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
  pub class StringListClass : ISerializable
  {
    pub Name: String;
    pub Length: i32;
    pub Width: i32;
    pub string[,] Data;
    pub ColumnName: Vec<String>;
    pub ID: i32;
    pub TempBmp: Vec<i32>;
    pub TempColumBmp: Vec<String>;
    pub string[,] TempDesc;
    pub LookUpCol: Vec<i32>;
    pub LookUpId: i32;
    pub LookUpLabel: i32;
    pub logEnabled: Vec<bool>;
    pub LibIdClass LibId;
    pub Description: String;
    pub Editable: bool;
    pub NewEnums.LibVarValueType[] ColValueType;
    pub ColSort: Vec<i32>;
    pub ColWidth: Vec<i32>;
    pub SSID: Vec<i32>;

    pub StringListClass Clone()
    {
      BinaryFormatter binaryFormatter = BinaryFormatter::new();
      MemoryStream serializationStream = MemoryStream::new();
      binaryFormatter.Serialize((Stream) serializationStream,  this);
      serializationStream.Position = 0L;
      return (StringListClass) binaryFormatter.Deserialize((Stream) serializationStream);
    }

    pub fn GetObjectData(SerializationInfo info, StreamingContext context)
    {
      info.AddValue("Name",  self.Name);
      info.AddValue("ColumnName",  self.ColumnName);
      info.AddValue("Length", self.Length);
      info.AddValue("Width", self.Width);
      info.AddValue("Data",  self.Data);
      info.AddValue("ID", self.ID);
      info.AddValue("LibId",  self.LibId);
      info.AddValue("Description",  self.Description);
      info.AddValue("ColValueType",  self.ColValueType);
      info.AddValue("Editable", self.Editable);
      info.AddValue("LookUpCol",  self.LookUpCol);
      info.AddValue("LookUpId", self.LookUpId);
      info.AddValue("LookUpLabel", self.LookUpLabel);
      info.AddValue("ColSort",  self.ColSort);
      info.AddValue("ColWidth",  self.ColWidth);
      info.AddValue("logEnabled",  self.logEnabled);
      info.AddValue("SSID",  self.SSID);
    }

    pub fn GetRandomId(idInCol: i32, weightInCol: i32) -> i32
    {
      SimpleList simpleList = SimpleList::new();
      let mut length: i32 = self.Length;
      for (let mut index: i32 = 0; index <= length; index += 1)
      {
        if (Conversions.ToDouble(self.Data[index, weightInCol]) > 0.0)
          simpleList.Add(Conversions.ToInteger(self.Data[index, idInCol]), Conversions.ToInteger(self.Data[index, weightInCol]));
      }
      if (simpleList.Counter <= -1)
        return -1;
      let mut index1: i32 = DrawMod.RandyNumber.Next(0, simpleList.Counter + 1);
      return simpleList.Id[index1];
    }

    pub fn GetHighestValue(col: i32, bool return0ifNoneFound = false) -> i32
    {
      let mut highestValue: i32 = -99999;
      let mut length: i32 = self.Length;
      for (let mut index: i32 = 0; index <= length; index += 1)
      {
        if ( Math.Round(Conversion.Val(self.Data[index, col])) > highestValue)
          highestValue =  Math.Round(Conversion.Val(self.Data[index, col]));
      }
      if (return0ifNoneFound && highestValue == -99999)
        highestValue = 0;
      return highestValue;
    }

    pub FindValue: bool(col: i32, tval: i32)
    {
      let mut length: i32 = self.Length;
      for (let mut index: i32 = 0; index <= length; index += 1)
      {
        if ( Math.Round(Conversion.Val(self.Data[index, col])) == tval)
          return true;
      }
      return false;
    }

    pub FindValue2: bool(col: i32, tval: String, col2: i32, tval2: String)
    {
      let mut length: i32 = self.Length;
      for (let mut index: i32 = 0; index <= length; index += 1)
      {
        if (Operators.CompareString(Strings.LCase(self.Data[index, col]), Strings.LCase(tval), false) == 0 && Operators.CompareString(Strings.LCase(self.Data[index, col2]), Strings.LCase(tval2), false) == 0)
          return true;
      }
      return false;
    }

    pub FindValue2: bool(col: i32, tval: i32, col2: i32, tval2: i32)
    {
      let mut length: i32 = self.Length;
      for (let mut index: i32 = 0; index <= length; index += 1)
      {
        if (Conversions.ToDouble(Strings.LCase(self.Data[index, col])) ==  tval && Conversions.ToDouble(Strings.LCase(self.Data[index, col2])) ==  tval2)
          return true;
      }
      return false;
    }

    pub fn FindRow(col: i32, tval: i32) -> i32
    {
      let mut length: i32 = self.Length;
      for (let mut row: i32 = 0; row <= length; row += 1)
      {
        if ( Math.Round(Conversion.Val(self.Data[row, col])) == tval)
          return row;
      }
      return -1;
    }

    pub fn FindRow(col: i32, tval: String) -> i32
    {
      let mut length: i32 = self.Length;
      for (let mut row: i32 = 0; row <= length; row += 1)
      {
        if (Operators.CompareString(Strings.LCase(self.Data[row, col]), Strings.LCase(tval), false) == 0)
          return row;
      }
      return -1;
    }

    pub fn FindRow2(col: i32, tval: i32, col2: i32, tval2: i32) -> i32
    {
      let mut length: i32 = self.Length;
      for (let mut row2: i32 = 0; row2 <= length; row2 += 1)
      {
        if ( Math.Round(Conversion.Val(self.Data[row2, col])) == tval &&  Math.Round(Conversion.Val(self.Data[row2, col2])) == tval2)
          return row2;
      }
      return -1;
    }

    pub fn FindRow3(col: i32, tval: i32, col2: i32, tval2: i32, col3: i32, tval3: i32) -> i32
    {
      let mut length: i32 = self.Length;
      for (let mut row3: i32 = 0; row3 <= length; row3 += 1)
      {
        if ( Math.Round(Conversion.Val(self.Data[row3, col])) == tval &&  Math.Round(Conversion.Val(self.Data[row3, col2])) == tval2 &&  Math.Round(Conversion.Val(self.Data[row3, col3])) == tval3)
          return row3;
      }
      return -1;
    }

    pub FindValue: bool(col: i32, tval: i32, col2: i32, tval2: i32)
    {
      let mut length: i32 = self.Length;
      for (let mut index: i32 = 0; index <= length; index += 1)
      {
        if ( Math.Round(Conversion.Val(self.Data[index, col])) == tval &&  Math.Round(Conversion.Val(self.Data[index, col2])) == tval2)
          return true;
      }
      return false;
    }

    pub GetData: String(idInCol: i32, idValue: i32, returnCol: i32)
    {
      let mut length: i32 = self.Length;
      for (let mut index: i32 = 0; index <= length; index += 1)
      {
        if ( Math.Round(Conversion.Val(self.Data[index, idInCol])) == idValue)
          return self.Data[index, returnCol];
      }
      return Conversions.ToString(-1);
    }

    pub fn GetDataCount(idInCol: i32, idValue: i32) -> i32
    {
      let mut length: i32 = self.Length;
      dataCount: i32;
      for (let mut index: i32 = 0; index <= length; index += 1)
      {
        if ( Math.Round(Conversion.Val(self.Data[index, idInCol])) == idValue)
          dataCount += 1;
      }
      return dataCount;
    }

    pub fn GetData2Count(idInCol: i32, idValue: i32, idInCol2: i32, idValue2: i32) -> i32
    {
      let mut length: i32 = self.Length;
      data2Count: i32;
      for (let mut index: i32 = 0; index <= length; index += 1)
      {
        if ( Math.Round(Conversion.Val(self.Data[index, idInCol])) == idValue &&  Math.Round(Conversion.Val(self.Data[index, idInCol2])) == idValue2)
          data2Count += 1;
      }
      return data2Count;
    }

    pub fn GetDataCountWeight(idInCol: i32, idValue: i32, weightCol: i32) -> i32
    {
      let mut length: i32 = self.Length;
      dataCountWeight: i32;
      for (let mut index: i32 = 0; index <= length; index += 1)
      {
        if ( Math.Round(Conversion.Val(self.Data[index, idInCol])) == idValue)
          dataCountWeight +=  Math.Round(Conversion.Val(self.Data[index, weightCol]));
      }
      return dataCountWeight;
    }

    pub GetData2CountWeight: i32(
      idInCol: i32,
      idValue: i32,
      idInCol2: i32,
      idValue2: i32,
      weightCol: i32)
    {
      let mut length: i32 = self.Length;
      data2CountWeight: i32;
      for (let mut index: i32 = 0; index <= length; index += 1)
      {
        if ( Math.Round(Conversion.Val(self.Data[index, idInCol])) == idValue &&  Math.Round(Conversion.Val(self.Data[index, idInCol2])) == idValue2)
          data2CountWeight +=  Math.Round(Conversion.Val(self.Data[index, weightCol]));
      }
      return data2CountWeight;
    }

    pub GetData3CountWeight: i32(
      idInCol: i32,
      idValue: i32,
      idInCol2: i32,
      idValue2: i32,
      idInCol3: i32,
      idValue3: i32,
      weightCol: i32)
    {
      let mut length: i32 = self.Length;
      data3CountWeight: i32;
      for (let mut index: i32 = 0; index <= length; index += 1)
      {
        if ( Math.Round(Conversion.Val(self.Data[index, idInCol])) == idValue &&  Math.Round(Conversion.Val(self.Data[index, idInCol2])) == idValue2 &&  Math.Round(Conversion.Val(self.Data[index, idInCol3])) == idValue3)
          data3CountWeight +=  Math.Round(Conversion.Val(self.Data[index, weightCol]));
      }
      return data3CountWeight;
    }

    pub GetData4CountWeight: i32(
      idInCol: i32,
      idValue: i32,
      idInCol2: i32,
      idValue2: i32,
      idInCol3: i32,
      idValue3: i32,
      idInCol4: i32,
      idValue4: i32,
      weightCol: i32)
    {
      let mut length: i32 = self.Length;
      data4CountWeight: i32;
      for (let mut index: i32 = 0; index <= length; index += 1)
      {
        if ( Math.Round(Conversion.Val(self.Data[index, idInCol])) == idValue &&  Math.Round(Conversion.Val(self.Data[index, idInCol2])) == idValue2 &&  Math.Round(Conversion.Val(self.Data[index, idInCol3])) == idValue3 &&  Math.Round(Conversion.Val(self.Data[index, idInCol4])) == idValue4)
          data4CountWeight +=  Math.Round(Conversion.Val(self.Data[index, weightCol]));
      }
      return data4CountWeight;
    }

    pub GetData: String(idInCol: i32, idValue: String, returnCol: i32)
    {
      let mut length: i32 = self.Length;
      for (let mut index: i32 = 0; index <= length; index += 1)
      {
        if (Operators.CompareString(Strings.LCase(self.Data[index, idInCol]), Strings.LCase(idValue), false) == 0)
          return self.Data[index, returnCol];
      }
      return Conversions.ToString(0);
    }

    pub GetData2: String(idInCol: i32, idValue: i32, idInCol2: i32, idValue2: i32, returnCol: i32)
    {
      let mut length: i32 = self.Length;
      for (let mut index: i32 = 0; index <= length; index += 1)
      {
        if ( Math.Round(Conversion.Val(self.Data[index, idInCol])) == idValue &&  Math.Round(Conversion.Val(self.Data[index, idInCol2])) == idValue2)
          return self.Data[index, returnCol];
      }
      return "0";
    }

    pub GetData2: String(
      idInCol: i32,
      idValue: String,
      idInCol2: i32,
      idValue2: i32,
      returnCol: i32)
    {
      let mut length: i32 = self.Length;
      for (let mut index: i32 = 0; index <= length; index += 1)
      {
        if (Operators.CompareString(self.Data[index, idInCol], idValue, false) == 0 &&  Math.Round(Conversion.Val(self.Data[index, idInCol2])) == idValue2)
          return self.Data[index, returnCol];
      }
      return "0";
    }

    pub GetData3: String(
      idInCol: i32,
      idValue: i32,
      idInCol2: i32,
      idValue2: i32,
      idInCol3: i32,
      idValue3: String,
      returnCol: i32)
    {
      if (self.Length == -1)
        return "0";
      let mut length: i32 = self.Length;
      for (let mut index: i32 = 0; index <= length; index += 1)
      {
        if ( Math.Round(Conversion.Val(self.Data[index, idInCol])) == idValue &&  Math.Round(Conversion.Val(self.Data[index, idInCol2])) == idValue2 && Operators.CompareString(Strings.LCase(self.Data[index, idInCol3]), Strings.LCase(idValue3), false) == 0)
          return self.Data[index, returnCol];
      }
      return "0";
    }

    pub GetData3: String(
      idInCol: i32,
      idValue: i32,
      idInCol2: i32,
      idValue2: String,
      idInCol3: i32,
      idValue3: i32,
      returnCol: i32)
    {
      let mut length: i32 = self.Length;
      for (let mut index: i32 = 0; index <= length; index += 1)
      {
        if ( Math.Round(Conversion.Val(self.Data[index, idInCol])) == idValue && Operators.CompareString(Strings.LCase(self.Data[index, idInCol2]), Strings.LCase(idValue2), false) == 0 &&  Math.Round(Conversion.Val(self.Data[index, idInCol3])) == idValue3)
          return self.Data[index, returnCol];
      }
      return "0";
    }

    pub GetData3: String(
      idInCol: i32,
      idValue: String,
      idInCol2: i32,
      idValue2: String,
      idInCol3: i32,
      idValue3: String,
      returnCol: i32)
    {
      let mut length: i32 = self.Length;
      for (let mut index: i32 = 0; index <= length; index += 1)
      {
        if (Operators.CompareString(Strings.LCase(self.Data[index, idInCol]), Strings.LCase(idValue), false) == 0 && Operators.CompareString(Strings.LCase(self.Data[index, idInCol2]), Strings.LCase(idValue2), false) == 0 && Operators.CompareString(Strings.LCase(self.Data[index, idInCol3]), Strings.LCase(idValue3), false) == 0)
          return self.Data[index, returnCol];
      }
      return "0";
    }

    pub GetData4: String(
      idInCol: i32,
      idValue: i32,
      idInCol2: i32,
      idValue2: i32,
      idInCol3: i32,
      idValue3: i32,
      idInCol4: i32,
      idValue4: i32,
      returnCol: i32)
    {
      let mut length: i32 = self.Length;
      for (let mut index: i32 = 0; index <= length; index += 1)
      {
        if ( Math.Round(Conversion.Val(self.Data[index, idInCol])) == idValue &&  Math.Round(Conversion.Val(self.Data[index, idInCol2])) == idValue2 &&  Math.Round(Conversion.Val(self.Data[index, idInCol3])) == idValue3 &&  Math.Round(Conversion.Val(self.Data[index, idInCol4])) == idValue4)
          return self.Data[index, returnCol];
      }
      return "0";
    }

    pub GetData5: String(
      idInCol: i32,
      idValue: i32,
      idInCol2: i32,
      idValue2: i32,
      idInCol3: i32,
      idValue3: i32,
      idInCol4: i32,
      idValue4: i32,
      idInCol5: i32,
      idValue5: i32,
      returnCol: i32)
    {
      let mut length: i32 = self.Length;
      for (let mut index: i32 = 0; index <= length; index += 1)
      {
        if ( Math.Round(Conversion.Val(self.Data[index, idInCol])) == idValue &&  Math.Round(Conversion.Val(self.Data[index, idInCol2])) == idValue2 &&  Math.Round(Conversion.Val(self.Data[index, idInCol3])) == idValue3 &&  Math.Round(Conversion.Val(self.Data[index, idInCol4])) == idValue4 &&  Math.Round(Conversion.Val(self.Data[index, idInCol5])) == idValue5)
          return self.Data[index, returnCol];
      }
      return "0";
    }

    pub GetData4: String(
      idInCol: i32,
      idValue: String,
      idInCol2: i32,
      idValue2: String,
      idInCol3: i32,
      idValue3: String,
      idInCol4: i32,
      idValue4: String,
      returnCol: i32)
    {
      let mut length: i32 = self.Length;
      for (let mut index: i32 = 0; index <= length; index += 1)
      {
        if (Operators.CompareString(Strings.LCase(self.Data[index, idInCol]), Strings.LCase(idValue), false) == 0 && Operators.CompareString(Strings.LCase(self.Data[index, idInCol2]), Strings.LCase(idValue2), false) == 0 && Operators.CompareString(Strings.LCase(self.Data[index, idInCol3]), Strings.LCase(idValue3), false) == 0 && Operators.CompareString(Strings.LCase(self.Data[index, idInCol4]), Strings.LCase(idValue4), false) == 0)
          return self.Data[index, returnCol];
      }
      return "0";
    }

    pub GetData3: String(
      idInCol: i32,
      idValue: i32,
      idInCol2: i32,
      idValue2: i32,
      idInCol3: i32,
      idValue3: i32,
      returnCol: i32)
    {
      let mut length: i32 = self.Length;
      for (let mut index: i32 = 0; index <= length; index += 1)
      {
        if ( Math.Round(Conversion.Val(self.Data[index, idInCol])) == idValue &&  Math.Round(Conversion.Val(self.Data[index, idInCol2])) == idValue2 &&  Math.Round(Conversion.Val(self.Data[index, idInCol3])) == idValue3)
          return self.Data[index, returnCol];
      }
      return "0";
    }

    pub GetData2: String(
      idInCol: i32,
      idValue: i32,
      idInCol2: i32,
      idValue2: String,
      returnCol: i32)
    {
      let mut length: i32 = self.Length;
      for (let mut index: i32 = 0; index <= length; index += 1)
      {
        if ( Math.Round(Conversion.Val(self.Data[index, idInCol])) == idValue && Operators.CompareString(Strings.LCase(self.Data[index, idInCol2]), Strings.LCase(idValue2), false) == 0)
          return self.Data[index, returnCol];
      }
      return "0";
    }

    pub void AddRowWithData(
      s0: String,
      s1: String = "",
      s2: String = "",
      s3: String = "",
      s4: String = "",
      s5: String = "",
      s6: String = "",
      s7: String = "",
      s8: String = "",
      s9: String = "",
      s10: String = "",
      s11: String = "",
      s12: String = "",
      s13: String = "",
      s14: String = "",
      s15: String = "",
      s16: String = "",
      s17: String = "",
      s18: String = "",
      s19: String = "",
      s20: String = "")
    {
      if (self.Length == -1)
        self.Length = self.Length;
      self.AddRowFast(self.Length);
      self.SetRowData(self.Length, s0, s1, s2, s3, s4, s5, s6, s7, s8, s9, s10, s11, s12, s13, s14, s15, s16, s17, s18, s19, s20);
    }

    pub fn AddRowWithData(s0: String, s1: String, s2: String, s3: String, s4: String)
    {
      self.AddRowFast(self.Length);
      self.SetRowData(self.Length, s0, s1, s2, s3, s4);
    }

    pub fn AddRowWithDataFast(s0: String, s1: String, s2: String, s3: String, s4: String)
    {
      self.AddRowFast(self.Length);
      self.SetRowData(self.Length, s0, s1, s2, s3, s4);
    }

    pub fn SetRowData(rowNr: i32, s0: String, s1: String, s2: String, s3: String, s4: String)
    {
      self.Data[rowNr, 0] = s0;
      self.Data[rowNr, 1] = s1;
      self.Data[rowNr, 2] = s2;
      self.Data[rowNr, 3] = s3;
      if (self.Width < 4)
        return;
      self.Data[rowNr, 4] = s4;
    }

    pub void SetRowData(
      rowNr: i32,
      s0: String,
      s1: String = "",
      s2: String = "",
      s3: String = "",
      s4: String = "",
      s5: String = "",
      s6: String = "",
      s7: String = "",
      s8: String = "",
      s9: String = "",
      s10: String = "",
      s11: String = "",
      s12: String = "",
      s13: String = "",
      s14: String = "",
      s15: String = "",
      s16: String = "",
      s17: String = "",
      s18: String = "",
      s19: String = "",
      s20: String = "")
    {
      if (self.Width >= 0)
        self.Data[rowNr, 0] = s0;
      if (self.Width >= 1)
        self.Data[rowNr, 1] = s1;
      if (self.Width >= 2)
        self.Data[rowNr, 2] = s2;
      if (self.Width >= 3)
        self.Data[rowNr, 3] = s3;
      if (self.Width >= 4)
        self.Data[rowNr, 4] = s4;
      if (self.Width >= 5)
        self.Data[rowNr, 5] = s5;
      if (self.Width >= 6)
        self.Data[rowNr, 6] = s6;
      if (self.Width >= 7)
        self.Data[rowNr, 7] = s7;
      if (self.Width >= 8)
        self.Data[rowNr, 8] = s8;
      if (self.Width >= 9)
        self.Data[rowNr, 9] = s9;
      if (self.Width >= 10)
        self.Data[rowNr, 10] = s10;
      if (self.Width >= 11)
        self.Data[rowNr, 11] = s11;
      if (self.Width >= 12)
        self.Data[rowNr, 12] = s12;
      if (self.Width >= 13)
        self.Data[rowNr, 13] = s13;
      if (self.Width >= 14)
        self.Data[rowNr, 14] = s14;
      if (self.Width >= 15)
        self.Data[rowNr, 15] = s15;
      if (self.Width >= 16)
        self.Data[rowNr, 16] = s16;
      if (self.Width >= 17)
        self.Data[rowNr, 17] = s17;
      if (self.Width >= 18)
        self.Data[rowNr, 18] = s18;
      if (self.Width >= 19)
        self.Data[rowNr, 19] = s19;
      if (self.Width < 20)
        return;
      self.Data[rowNr, 20] = s20;
    }

    pub fn SetData(idInCol: i32, idValue: i32, setCol: i32, setValue: i32, bool allowAdd = false)
    {
      let mut length: i32 = self.Length;
      for (let mut index: i32 = 0; index <= length; index += 1)
      {
        if ( Math.Round(Conversion.Val(self.Data[index, idInCol])) == idValue)
        {
          self.Data[index, setCol] = setValue.ToString();
          return;
        }
      }
      if (!allowAdd)
        return;
      self.AddRow(self.Length);
      self.Data[self.Length, idInCol] = idValue.ToString();
      self.Data[self.Length, setCol] = setValue.ToString();
    }

    pub fn SetData(idInCol: i32, idValue: String, setCol: i32, setValue: i32, bool allowAdd = false)
    {
      let mut length: i32 = self.Length;
      for (let mut index: i32 = 0; index <= length; index += 1)
      {
        if (Operators.CompareString(Strings.LCase(self.Data[index, idInCol]), Strings.LCase(idValue), false) == 0)
        {
          self.Data[index, setCol] = setValue.ToString();
          return;
        }
      }
      if (!allowAdd)
        return;
      self.AddRow(self.Length);
      self.Data[self.Length, idInCol] = idValue;
      self.Data[self.Length, setCol] = setValue.ToString();
    }

    pub fn SetData(idInCol: i32, idValue: String, setCol: i32, setValue: String, bool allowAdd = false)
    {
      let mut length: i32 = self.Length;
      for (let mut index: i32 = 0; index <= length; index += 1)
      {
        if (Operators.CompareString(Strings.LCase(self.Data[index, idInCol]), Strings.LCase(idValue), false) == 0)
        {
          self.Data[index, setCol] = setValue;
          return;
        }
      }
      if (!allowAdd)
        return;
      self.AddRow(self.Length);
      self.Data[self.Length, idInCol] = idValue;
      self.Data[self.Length, setCol] = setValue;
    }

    pub void SetData2(
      idInCol: i32,
      idValue: i32,
      idInCol2: i32,
      idValue2: i32,
      setCol: i32,
      setValue: i32,
      bool allowAdd = false)
    {
      let mut length: i32 = self.Length;
      for (let mut index: i32 = 0; index <= length; index += 1)
      {
        if ( Math.Round(Conversion.Val(self.Data[index, idInCol])) == idValue &&  Math.Round(Conversion.Val(self.Data[index, idInCol2])) == idValue2)
        {
          self.Data[index, setCol] = setValue.ToString();
          return;
        }
      }
      if (!allowAdd)
        return;
      self.AddRow(self.Length);
      self.Data[self.Length, idInCol] = idValue.ToString();
      self.Data[self.Length, idInCol2] = idValue2.ToString();
      self.Data[self.Length, setCol] = setValue.ToString();
    }

    pub void SetData3(
      idInCol: i32,
      idValue: i32,
      idInCol2: i32,
      idValue2: i32,
      idInCol3: i32,
      idValue3: i32,
      setCol: i32,
      setValue: i32,
      bool allowAdd = false)
    {
      let mut length: i32 = self.Length;
      for (let mut index: i32 = 0; index <= length; index += 1)
      {
        if ( Math.Round(Conversion.Val(self.Data[index, idInCol])) == idValue &&  Math.Round(Conversion.Val(self.Data[index, idInCol2])) == idValue2 &&  Math.Round(Conversion.Val(self.Data[index, idInCol3])) == idValue3)
        {
          self.Data[index, setCol] = setValue.ToString();
          return;
        }
      }
      if (!allowAdd)
        return;
      self.AddRow(self.Length);
      self.Data[self.Length, idInCol] = idValue.ToString();
      self.Data[self.Length, idInCol2] = idValue2.ToString();
      self.Data[self.Length, idInCol3] = idValue3.ToString();
      self.Data[self.Length, setCol] = setValue.ToString();
    }

    pub void SetData3(
      idInCol: i32,
      idValue: i32,
      idInCol2: i32,
      idValue2: i32,
      idInCol3: i32,
      idValue3: String,
      setCol: i32,
      setValue: i32,
      bool allowAdd = false)
    {
      let mut length: i32 = self.Length;
      for (let mut index: i32 = 0; index <= length; index += 1)
      {
        if ( Math.Round(Conversion.Val(self.Data[index, idInCol])) == idValue &&  Math.Round(Conversion.Val(self.Data[index, idInCol2])) == idValue2 && Operators.CompareString(Strings.LCase(self.Data[index, idInCol3]), Strings.LCase(idValue3), false) == 0)
        {
          self.Data[index, setCol] = setValue.ToString();
          return;
        }
      }
      if (!allowAdd)
        return;
      self.AddRow(self.Length);
      self.Data[self.Length, idInCol] = idValue.ToString();
      self.Data[self.Length, idInCol2] = idValue2.ToString();
      self.Data[self.Length, idInCol3] = idValue3.ToString();
      self.Data[self.Length, setCol] = setValue.ToString();
    }

    pub void SetData4(
      idInCol: i32,
      idValue: i32,
      idInCol2: i32,
      idValue2: i32,
      idInCol3: i32,
      idValue3: i32,
      idInCol4: i32,
      idValue4: i32,
      setCol: i32,
      setValue: i32,
      bool allowAdd = false)
    {
      let mut length: i32 = self.Length;
      for (let mut index: i32 = 0; index <= length; index += 1)
      {
        if ( Math.Round(Conversion.Val(self.Data[index, idInCol])) == idValue &&  Math.Round(Conversion.Val(self.Data[index, idInCol2])) == idValue2 &&  Math.Round(Conversion.Val(self.Data[index, idInCol3])) == idValue3 &&  Math.Round(Conversion.Val(self.Data[index, idInCol4])) == idValue4)
        {
          self.Data[index, setCol] = setValue.ToString();
          return;
        }
      }
      if (!allowAdd)
        return;
      self.AddRow(self.Length);
      self.Data[self.Length, idInCol] = idValue.ToString();
      self.Data[self.Length, idInCol2] = idValue2.ToString();
      self.Data[self.Length, idInCol3] = idValue3.ToString();
      self.Data[self.Length, idInCol4] = idValue4.ToString();
      self.Data[self.Length, setCol] = setValue.ToString();
    }

    pub void SetData4(
      idInCol: i32,
      idValue: String,
      idInCol2: i32,
      idValue2: String,
      idInCol3: i32,
      idValue3: String,
      idInCol4: i32,
      idValue4: String,
      setCol: i32,
      setValue: i32,
      bool allowAdd = false)
    {
      let mut length: i32 = self.Length;
      for (let mut index: i32 = 0; index <= length; index += 1)
      {
        if (Operators.CompareString(Strings.LCase(self.Data[index, idInCol]), Strings.LCase(idValue), false) == 0 && Operators.CompareString(Strings.LCase(self.Data[index, idInCol2]), Strings.LCase(idValue2), false) == 0 && Operators.CompareString(Strings.LCase(self.Data[index, idInCol3]), Strings.LCase(idValue3), false) == 0 && Operators.CompareString(Strings.LCase(self.Data[index, idInCol4]), Strings.LCase(idValue4), false) == 0)
        {
          self.Data[index, setCol] = setValue.ToString();
          return;
        }
      }
      if (!allowAdd)
        return;
      self.AddRow(self.Length);
      self.Data[self.Length, idInCol] = idValue;
      self.Data[self.Length, idInCol2] = idValue2;
      self.Data[self.Length, idInCol3] = idValue3;
      self.Data[self.Length, idInCol4] = idValue4;
      self.Data[self.Length, setCol] = Conversions.ToString(setValue);
    }

    pub void SetData2(
      idInCol: i32,
      idValue: i32,
      idInCol2: i32,
      idValue2: String,
      setCol: i32,
      setValue: i32,
      bool allowAdd = false)
    {
      let mut length: i32 = self.Length;
      for (let mut index: i32 = 0; index <= length; index += 1)
      {
        if ( Math.Round(Conversion.Val(self.Data[index, idInCol])) == idValue && Operators.CompareString(Strings.LCase(self.Data[index, idInCol2]), Strings.LCase(idValue2), false) == 0)
        {
          self.Data[index, setCol] = setValue.ToString();
          return;
        }
      }
      if (!allowAdd)
        return;
      self.AddRow(self.Length);
      self.Data[self.Length, idInCol] = idValue.ToString();
      self.Data[self.Length, idInCol2] = idValue2;
      self.Data[self.Length, setCol] = setValue.ToString();
    }

    pub fn SetData(idInCol: i32, idValue: i32, setCol: i32, setValue: String, bool allowAdd = false)
    {
      let mut length: i32 = self.Length;
      for (let mut index: i32 = 0; index <= length; index += 1)
      {
        if ( Math.Round(Conversion.Val(self.Data[index, idInCol])) == idValue)
        {
          self.Data[index, setCol] = setValue;
          return;
        }
      }
      if (!allowAdd)
        return;
      self.AddRow(self.Length);
      self.Data[self.Length, idInCol] = idValue.ToString();
      self.Data[self.Length, setCol] = setValue;
    }

    protected StringListClass(SerializationInfo info, StreamingContext context)
    {
      self.Data = new string[1, 1];
      self.ColumnName = new string[1];
      self.TempBmp = new int[1, 1];
      self.TempColumBmp = new string[1];
      self.TempDesc = new string[1, 1];
      self.LookUpCol = new int[1];
      self.logEnabled = new bool[1];
      self.ColValueType = new NewEnums.LibVarValueType[1];
      self.ColSort = new int[1];
      self.ColWidth = new int[1];
      self.SSID = new int[1];
      self.Name = info.GetString(nameof (Name));
      self.Length = info.GetInt32(nameof (Length));
      self.Width = info.GetInt32(nameof (Width));
      self.Data = new string[self.Length + 1, self.Width + 1];
      self.TempBmp = new int[self.Length + 1, self.Width + 1];
      self.TempDesc = new string[self.Length + 1, self.Width + 1];
      self.ColumnName = new string[self.Width + 1];
      self.logEnabled = new bool[self.Width + 1];
      self.ColSort = new int[self.Width + 1];
      self.ColWidth = new int[self.Width + 1];
      self.TempColumBmp = new string[self.Width + 1];
      self.Data = (string[,]) info.GetValue(nameof (Data), self.Data.GetType());
      self.ColumnName = (string[]) info.GetValue(nameof (ColumnName), self.ColumnName.GetType());
      try
      {
        self.logEnabled = (bool[]) info.GetValue(nameof (logEnabled), self.logEnabled.GetType());
      }
      catch (Exception ex)
      {
        ProjectData.SetProjectError(ex);
        let mut width: i32 = self.Width;
        for (let mut index: i32 = 0; index <= width; index += 1)
          self.logEnabled[index] = false;
        ProjectData.ClearProjectError();
      }
      self.ID = info.GetInt32(nameof (ID));
      let mut upperBound1: i32 = self.Data.GetUpperBound(0);
      for (let mut index1: i32 = 0; index1 <= upperBound1; index1 += 1)
      {
        let mut upperBound2: i32 = self.Data.GetUpperBound(1);
        for (let mut index2: i32 = 0; index2 <= upperBound2; index2 += 1)
        {
          if (Information.IsNothing( self.Data[index1, index2]))
            self.Data[index1, index2] = "";
        }
      }
      self.ColValueType = new NewEnums.LibVarValueType[self.Width + 1];
      try
      {
        self.LibId = LibIdClass::new();
        self.LibId = (LibIdClass) info.GetValue(nameof (LibId), self.LibId.GetType());
        self.Description = info.GetString(nameof (Description));
        self.ColValueType = (NewEnums.LibVarValueType[]) info.GetValue(nameof (ColValueType), self.ColValueType.GetType());
      }
      catch (Exception ex)
      {
        ProjectData.SetProjectError(ex);
        self.LibId = LibIdClass::new();
        self.Description = "No description";
        let mut width: i32 = self.Width;
        for (let mut index: i32 = 0; index <= width; index += 1)
          self.ColValueType[index] = NewEnums.LibVarValueType.Text;
        ProjectData.ClearProjectError();
      }
      try
      {
        self.Editable = info.GetBoolean(nameof (Editable));
      }
      catch (Exception ex)
      {
        ProjectData.SetProjectError(ex);
        self.Editable = true;
        ProjectData.ClearProjectError();
      }
      self.LookUpCol = new int[self.Width + 1];
      try
      {
        self.LookUpId = info.GetInt32(nameof (LookUpId));
        self.LookUpLabel = info.GetInt32(nameof (LookUpLabel));
        self.LookUpCol = (int[]) info.GetValue(nameof (LookUpCol), self.LookUpCol.GetType());
      }
      catch (Exception ex)
      {
        ProjectData.SetProjectError(ex);
        self.LookUpId = -1;
        self.LookUpLabel = -1;
        let mut width: i32 = self.Width;
        for (let mut index: i32 = 0; index <= width; index += 1)
          self.LookUpCol[index] = -1;
        ProjectData.ClearProjectError();
      }
      try
      {
        self.ColSort = (int[]) info.GetValue(nameof (ColSort), self.ColSort.GetType());
        self.ColWidth = (int[]) info.GetValue(nameof (ColWidth), self.ColWidth.GetType());
      }
      catch (Exception ex)
      {
        ProjectData.SetProjectError(ex);
        ProjectData.ClearProjectError();
      }
      self.SSID = new int[self.Width + 1];
      try
      {
        self.SSID = (int[]) info.GetValue(nameof (SSID), self.SSID.GetType());
      }
      catch (Exception ex)
      {
        ProjectData.SetProjectError(ex);
        let mut width: i32 = self.Width;
        for (let mut index: i32 = 0; index <= width; index += 1)
          self.SSID[index] = 0;
        ProjectData.ClearProjectError();
      }
    }

    pub StringListClass(tid: i32)
    {
      self.Data = new string[1, 1];
      self.ColumnName = new string[1];
      self.TempBmp = new int[1, 1];
      self.TempColumBmp = new string[1];
      self.TempDesc = new string[1, 1];
      self.LookUpCol = new int[1];
      self.logEnabled = new bool[1];
      self.ColValueType = new NewEnums.LibVarValueType[1];
      self.ColSort = new int[1];
      self.ColWidth = new int[1];
      self.SSID = new int[1];
      self.Length = -1;
      self.ID = tid;
      self.Name = "New Stringlist";
      self.Width = -1;
      self.Data = new string[1, 1];
      self.ColumnName = new string[1];
      self.ColSort = new int[1];
      self.ColWidth = new int[1];
      self.LookUpCol = new int[1];
      self.SSID = new int[1];
      self.ColValueType = new NewEnums.LibVarValueType[1];
      self.logEnabled = new bool[1];
      self.Data[0, 0] = "";
      self.ColumnName[0] = "";
      self.logEnabled[0] = false;
      self.LibId = LibIdClass::new();
      self.Description = "No description";
      let mut width: i32 = self.Width;
      for (let mut index: i32 = 0; index <= width; index += 1)
        self.ColValueType[index] = NewEnums.LibVarValueType.Text;
      self.Editable = true;
      self.LookUpCol[0] = -1;
      self.LookUpId = -1;
      self.LookUpLabel = -1;
      self.SSID[0] = 0;
    }

    pub fn ClearAllRows()
    {
      self.Length = -1;
      self.Data = new string[self.Width + 1, 1];
    }

    pub fn Clear()
    {
      while (self.Length > -1)
        self.RemoveRow(self.Length);
      while (self.Width > -1)
        self.RemoveCol(self.Width);
    }

    pub LibVarClass GetValue( tData: DataClass, row: i32, col: i32, let mut useLibSlot: i32 = -1)
    {
      LibVarClass libVarClass = new LibVarClass(-1);
      if (col > -1)
      {
        if (self.LookUpCol[col] > 0)
        {
          let mut stringListById: i32 = DrawMod.TGame.HandyFunctionsObj.GetStringListByID(self.LookUpCol[col]);
          str1: String = self.Data[row, col];
          let mut num1: i32 =  Math.Round(Conversion.Val(str1));
          if (stringListById > -1 && tData.StringListObj[stringListById].LookUpId > -1 & tData.StringListObj[stringListById].LookUpLabel > -1)
          {
            if (Strings.InStr(str1, "[") > 0 & Strings.InStr(str1, "]") > 0)
            {
              str2: String = "";
              while (Strings.InStr(str1, "[") > 0 & Strings.InStr(str1, "]") > Strings.InStr(str1, "["))
              {
                let mut num2: i32 = Strings.InStr(str1, "[");
                let mut num3: i32 = Strings.InStr(str1, "]");
                let mut num4: i32 =  Math.Round(Conversion.Val(Strings.Mid(str1, num2 + 1, num3 - num2 - 1)));
                str3: String = "";
                let mut length: i32 = DrawMod.TGame.Data.StringListObj[stringListById].Length;
                for (let mut index: i32 = 0; index <= length; index += 1)
                {
                  if ( Math.Round(Conversion.Val(DrawMod.TGame.Data.StringListObj[stringListById].Data[index, tData.StringListObj[stringListById].LookUpId])) == num4)
                  {
                    str3 = "[" + num4.ToString() + "] " + DrawMod.TGame.Data.StringListObj[stringListById].Data[index, tData.StringListObj[stringListById].LookUpLabel];
                    break;
                  }
                }
                str1 = Strings.Mid(str1, num3 + 1);
                if (str3.Length > 16)
                  str3 = Strings.Left(str3, 16);
                if (str2.Length > 0)
                  str2 += ",";
                str2 += str3;
              }
              if (str2.Length > 0)
              {
                libVarClass.valueType = NewEnums.LibVarValueType.Text;
                libVarClass.value = -1;
                libVarClass.valueText = str2;
                return libVarClass;
              }
            }
            else
            {
              let mut length: i32 = DrawMod.TGame.Data.StringListObj[stringListById].Length;
              for (let mut index: i32 = 0; index <= length; index += 1)
              {
                if ( Math.Round(Conversion.Val(DrawMod.TGame.Data.StringListObj[stringListById].Data[index, tData.StringListObj[stringListById].LookUpId])) == num1)
                {
                  libVarClass.valueType = NewEnums.LibVarValueType.Text;
                  libVarClass.value = -1;
                  libVarClass.valueText = "[" + num1.ToString() + "] " + DrawMod.TGame.Data.StringListObj[stringListById].Data[index, tData.StringListObj[stringListById].LookUpLabel];
                  return libVarClass;
                }
              }
            }
          }
          libVarClass.valueType = NewEnums.LibVarValueType.Text;
          libVarClass.value = -1;
          libVarClass.valueText = self.Data[row, col];
          return libVarClass;
        }
        if (self.ColValueType[col] == NewEnums.LibVarValueType.Number)
        {
          libVarClass.valueType = NewEnums.LibVarValueType.Number;
          libVarClass.value =  Math.Round(Conversion.Val(self.Data[row, col]));
          libVarClass.valueText = self.Data[row, col];
        }
        else if (self.ColValueType[col] == NewEnums.LibVarValueType.DateString)
        {
          libVarClass.valueType = NewEnums.LibVarValueType.DateString;
          libVarClass.value = -1;
          libVarClass.valueText = self.Data[row, col];
        }
        else if (self.ColValueType[col] == NewEnums.LibVarValueType.Text)
        {
          libVarClass.valueType = NewEnums.LibVarValueType.Text;
          libVarClass.value = -1;
          libVarClass.valueText = self.Data[row, col];
        }
        else if (self.ColValueType[col] == NewEnums.LibVarValueType.RegimeId)
        {
          libVarClass.value =  Math.Round(Conversion.Val(self.Data[row, col]));
          libVarClass.valueType = NewEnums.LibVarValueType.RegimeId;
          if (libVarClass.value == 0 & Strings.InStr(self.Data[row, col], "0") < 1)
          {
            libVarClass.value = -1;
            self.Data[row, col] = "-1";
          }
          libVarClass.valueText = !(libVarClass.value > -1 & libVarClass.value <= tData.RegimeCounter) ? (libVarClass.value <= tData.RegimeCounter ? (libVarClass.value != -1 ? "Invalid value" : "None/All") : "Non-existing!") : tData.RegimeObj[libVarClass.value].Name;
        }
        else if (self.ColValueType[col] == NewEnums.LibVarValueType.HistoricalUnitId | self.ColValueType[col] == NewEnums.LibVarValueType.HistoricalUnitModelId)
        {
          libVarClass.value =  Math.Round(Conversion.Val(self.Data[row, col]));
          libVarClass.valueType = self.ColValueType[col];
          if (libVarClass.value == 0 & Strings.InStr(self.Data[row, col], "0") < 1)
          {
            libVarClass.value = -1;
            self.Data[row, col] = "-1";
          }
          libVarClass.valueText = !(libVarClass.value > -1 & libVarClass.value <= tData.HistoricalUnitCounter) ? (libVarClass.value <= tData.HistoricalUnitCounter ? (libVarClass.value != -1 ? "Invalid value" : "None/All") : "Non-existing!") : tData.HistoricalUnitObj[libVarClass.value].Name;
        }
        else if (self.ColValueType[col] == NewEnums.LibVarValueType.OfficerId)
        {
          libVarClass.value =  Math.Round(Conversion.Val(self.Data[row, col]));
          libVarClass.valueType = self.ColValueType[col];
          if (libVarClass.value == 0 & Strings.InStr(self.Data[row, col], "0") < 1)
          {
            libVarClass.value = -1;
            self.Data[row, col] = "-1";
          }
          libVarClass.valueText = !(libVarClass.value > -1 & libVarClass.value <= tData.HistoricalUnitCounter) ? (libVarClass.value <= tData.HistoricalUnitCounter ? (libVarClass.value != -1 ? "Invalid value" : "None/All") : "Non-existing!") : tData.HistoricalUnitObj[libVarClass.value].CommanderName;
        }
        else if (self.ColValueType[col] == NewEnums.LibVarValueType.LandscapeId)
        {
          libVarClass.value =  Math.Round(Conversion.Val(self.Data[row, col]));
          libVarClass.valueType = NewEnums.LibVarValueType.LandscapeId;
          if (libVarClass.value == 0 & Strings.InStr(self.Data[row, col], "0") < 1)
          {
            libVarClass.value = -1;
            self.Data[row, col] = "-1";
          }
          libVarClass.valueText = !(libVarClass.value > -1 & libVarClass.value <= tData.LandscapeTypeCounter) ? (libVarClass.value <= tData.LandscapeTypeCounter ? (libVarClass.value != -1 ? "Invalid value" : "None/All") : "Non-existing!") : tData.LandscapeTypeObj[libVarClass.value].Name;
        }
        else if (self.ColValueType[col] == NewEnums.LibVarValueType.ActionCardId)
        {
          libVarClass.value =  Math.Round(Conversion.Val(self.Data[row, col]));
          libVarClass.valueType = NewEnums.LibVarValueType.ActionCardId;
          if (libVarClass.value == 0 & Strings.InStr(self.Data[row, col], "0") < 1)
          {
            libVarClass.value = -1;
            self.Data[row, col] = "-1";
          }
          libVarClass.valueText = !(libVarClass.value > -1 & libVarClass.value <= tData.ActionCardCounter) ? (libVarClass.value <= tData.ActionCardCounter ? (libVarClass.value != -1 ? "Invalid value" : "None/All") : "Non-existing!") : tData.ActionCardObj[libVarClass.value].Title;
        }
        else if (self.ColValueType[col] == NewEnums.LibVarValueType.EventPicId)
        {
          libVarClass.value =  Math.Round(Conversion.Val(self.Data[row, col]));
          libVarClass.valueType = NewEnums.LibVarValueType.EventPicId;
          if (useLibSlot > -1 & DrawMod.TGame.Data.Product == 7)
          {
            if (libVarClass.value == 0 & Strings.InStr(self.Data[row, col], "0") < 1)
            {
              libVarClass.value = -1;
              self.Data[row, col] = "-1";
            }
            if (libVarClass.value > -1)
            {
              let mut eventPic: i32 = tData.FindEventPic(libVarClass.value, tData.LibraryObj[useLibSlot].name);
              libVarClass.valueText = eventPic <= -1 ? "Invalid value" : tData.EventPicName[eventPic];
            }
            else
              libVarClass.valueText = libVarClass.value != -1 ? "Invalid value" : "None/All";
          }
          else
          {
            if (libVarClass.value == 0 & Strings.InStr(self.Data[row, col], "0") < 1)
            {
              libVarClass.value = -1;
              self.Data[row, col] = "-1";
            }
            libVarClass.valueText = !(libVarClass.value > -1 & libVarClass.value <= tData.EventPicCounter) ? (libVarClass.value <= tData.EventPicCounter ? (libVarClass.value != -1 ? "Invalid value" : "None/All") : "Non-existing!") : tData.EventPicName[libVarClass.value];
          }
        }
        else if (self.ColValueType[col] == NewEnums.LibVarValueType.SmallGfxId)
        {
          libVarClass.value =  Math.Round(Conversion.Val(self.Data[row, col]));
          libVarClass.valueType = NewEnums.LibVarValueType.SmallGfxId;
          if (libVarClass.value == 0 & Strings.InStr(self.Data[row, col], "0") < 1)
          {
            libVarClass.value = -1;
            self.Data[row, col] = "-1";
          }
          libVarClass.valueText = !(libVarClass.value > -1 & libVarClass.value <= tData.SmallPicCounter) ? (libVarClass.value <= tData.SmallPicCounter ? (libVarClass.value != -1 ? "Invalid value" : "None/All") : "Non-existing!") : tData.SmallPicName[libVarClass.value];
        }
        else if (self.ColValueType[col] == NewEnums.LibVarValueType.PeopleId)
        {
          libVarClass.value =  Math.Round(Conversion.Val(self.Data[row, col]));
          libVarClass.valueType = NewEnums.LibVarValueType.PeopleId;
          if (libVarClass.value == 0 & Strings.InStr(self.Data[row, col], "0") < 1)
          {
            libVarClass.value = -1;
            self.Data[row, col] = "-1";
          }
          libVarClass.valueText = !(libVarClass.value > -1 & libVarClass.value <= tData.PeopleCounter) ? (libVarClass.value <= tData.PeopleCounter ? (libVarClass.value != -1 ? "Invalid value" : "None/All") : "Non-existing!") : tData.PeopleObj[libVarClass.value].Name;
        }
        else if (self.ColValueType[col] == NewEnums.LibVarValueType.RiverId)
        {
          libVarClass.value =  Math.Round(Conversion.Val(self.Data[row, col]));
          libVarClass.valueType = NewEnums.LibVarValueType.RiverId;
          if (libVarClass.value == 0 & Strings.InStr(self.Data[row, col], "0") < 1)
          {
            libVarClass.value = -1;
            self.Data[row, col] = "-1";
          }
          libVarClass.valueText = !(libVarClass.value > -1 & libVarClass.value <= tData.RiverTypeCounter) ? (libVarClass.value <= tData.RiverTypeCounter ? (libVarClass.value != -1 ? "Invalid value" : "None/All") : "Non-existing!") : tData.RiverTypeObj[libVarClass.value].Name;
        }
        else if (self.ColValueType[col] == NewEnums.LibVarValueType.RoadId)
        {
          libVarClass.value =  Math.Round(Conversion.Val(self.Data[row, col]));
          libVarClass.valueType = NewEnums.LibVarValueType.RoadId;
          if (libVarClass.value == 0 & Strings.InStr(self.Data[row, col], "0") < 1)
          {
            libVarClass.value = -1;
            self.Data[row, col] = "-1";
          }
          libVarClass.valueText = !(libVarClass.value > -1 & libVarClass.value <= tData.RoadTypeCounter) ? (libVarClass.value <= tData.RoadTypeCounter ? (libVarClass.value != -1 ? "Invalid value" : "None/All") : "Non-existing!") : tData.RoadTypeObj[libVarClass.value].Name;
        }
        else if (self.ColValueType[col] == NewEnums.LibVarValueType.SFTypeId)
        {
          libVarClass.value =  Math.Round(Conversion.Val(self.Data[row, col]));
          libVarClass.valueType = NewEnums.LibVarValueType.SFTypeId;
          if (libVarClass.value == 0 & Strings.InStr(self.Data[row, col], "0") < 1)
          {
            libVarClass.value = -1;
            self.Data[row, col] = "-1";
          }
          libVarClass.valueText = !(libVarClass.value > -1 & libVarClass.value <= tData.SFTypeCounter) ? (libVarClass.value <= tData.SFTypeCounter ? (libVarClass.value != -1 ? "Invalid value" : "None/All") : "Non-existing!") : tData.SFTypeObj[libVarClass.value].Name;
        }
        else if (self.ColValueType[col] == NewEnums.LibVarValueType.LocationTypeId)
        {
          libVarClass.value =  Math.Round(Conversion.Val(self.Data[row, col]));
          libVarClass.valueType = NewEnums.LibVarValueType.LocationTypeId;
          if (libVarClass.value == 0 & Strings.InStr(self.Data[row, col], "0") < 1)
          {
            libVarClass.value = -1;
            self.Data[row, col] = "-1";
          }
          libVarClass.valueText = !(libVarClass.value > -1 & libVarClass.value <= tData.LocTypeCounter) ? (libVarClass.value <= tData.LocTypeCounter ? (libVarClass.value != -1 ? "Invalid value" : "None/All") : "Non-existing!") : tData.LocTypeObj[libVarClass.value].Name;
        }
        else if (self.ColValueType[col] == NewEnums.LibVarValueType.YesNo)
        {
          libVarClass.value =  Math.Round(Conversion.Val(self.Data[row, col]));
          libVarClass.valueType = NewEnums.LibVarValueType.YesNo;
          libVarClass.valueText = libVarClass.value != 1 ? "No" : "Yes";
        }
      }
      return libVarClass;
    }

    pub fn SetItem(row: i32, col: i32, s: String)
    {
      while (row > self.Length)
        self.AddRow(self.Length);
      num: i32;
      if (col > self.Width)
      {
        self.Width = col;
        num = 1;
      }
      if (num == 1)
        self.Data = (string[,]) Utils.CopyArray((Array) self.Data, (Array) new string[self.Length + 1, self.Width + 1]);
      if (num == 1)
        self.TempBmp = (int[,]) Utils.CopyArray((Array) self.TempBmp, (Array) new int[self.Length + 1, self.Width + 1]);
      if (num == 1)
        self.TempDesc = (string[,]) Utils.CopyArray((Array) self.TempDesc, (Array) new string[self.Length + 1, self.Width + 1]);
      if (num == 1)
        self.ColValueType = (NewEnums.LibVarValueType[]) Utils.CopyArray((Array) self.ColValueType, (Array) new NewEnums.LibVarValueType[self.Width + 1]);
      if (num == 1)
        self.ColumnName = (string[]) Utils.CopyArray((Array) self.ColumnName, (Array) new string[self.Width + 1]);
      if (num == 1)
        self.ColSort = (int[]) Utils.CopyArray((Array) self.ColSort, (Array) new int[self.Width + 1]);
      if (num == 1)
        self.ColWidth = (int[]) Utils.CopyArray((Array) self.ColWidth, (Array) new int[self.Width + 1]);
      self.Data[row, col] = s;
    }

    pub fn SetColName(col: i32, s: String)
    {
      num: i32;
      if (col > self.Width)
      {
        self.Width = col;
        num = 1;
      }
      if (num == 1)
      {
        self.Data = (string[,]) Utils.CopyArray((Array) self.Data, (Array) new string[self.Length + 1, self.Width + 1]);
        self.logEnabled = (bool[]) Utils.CopyArray((Array) self.logEnabled, (Array) new bool[self.Width + 1]);
        self.ColumnName = (string[]) Utils.CopyArray((Array) self.ColumnName, (Array) new string[self.Width + 1]);
        self.ColSort = (int[]) Utils.CopyArray((Array) self.ColSort, (Array) new int[self.Width + 1]);
        self.ColWidth = (int[]) Utils.CopyArray((Array) self.ColWidth, (Array) new int[self.Width + 1]);
      }
      if (num == 1)
      {
        self.TempBmp = (int[,]) Utils.CopyArray((Array) self.TempBmp, (Array) new int[self.Length + 1, self.Width + 1]);
        self.TempColumBmp = (string[]) Utils.CopyArray((Array) self.TempColumBmp, (Array) new string[self.Width + 1]);
      }
      if (num == 1)
        self.TempDesc = (string[,]) Utils.CopyArray((Array) self.TempDesc, (Array) new string[self.Length + 1, self.Width + 1]);
      if (num == 1)
        self.ColValueType = (NewEnums.LibVarValueType[]) Utils.CopyArray((Array) self.ColValueType, (Array) new NewEnums.LibVarValueType[self.Width + 1]);
      self.ColumnName[col] = s;
      self.logEnabled[col] = false;
    }

    pub fn Sort(col: i32)
    {
      self.ColSort[col] = self.ColSort[col] > 1 ? 1 : 2;
      bool flag1 = true;
      while (flag1)
      {
        flag1 = false;
        let mut num1: i32 = self.Length - 1;
        for (let mut index1: i32 = 0; index1 <= num1; index1 += 1)
        {
          bool flag2 = false;
          str1: String = self.Data[index1, col];
          str2: String = self.Data[index1 + 1, col];
          let mut num2: i32 = Strings.InStr(str1, "#");
          let mut num3: i32 = Strings.InStr(num2 + 1, str1, "#");
          if (num2 > 0 & num3 > 0)
            str1 = Strings.Mid(str1, num2 + 1, num3 - num2 - 1);
          let mut num4: i32 = Strings.InStr(str2, "#");
          let mut num5: i32 = Strings.InStr(num4 + 1, str2, "#");
          if (num4 > 0 & num5 > 0)
            str2 = Strings.Mid(str2, num4 + 1, num5 - num4 - 1);
          if (self.ColValueType[col] == NewEnums.LibVarValueType.Number)
          {
            if (self.ColSort[col] == 1)
            {
              if ( Math.Round(Conversion.Val(str1)) >  Math.Round(Conversion.Val(str2)))
                flag2 = true;
            }
            else if (self.ColSort[col] == 2 &&  Math.Round(Conversion.Val(str1)) <  Math.Round(Conversion.Val(str2)))
              flag2 = true;
          }
          if (self.ColValueType[col] == NewEnums.LibVarValueType.Text)
          {
            if (self.ColSort[col] == 1)
            {
              if (string.Compare(str1, str2) == -1)
                flag2 = true;
            }
            else if (self.ColSort[col] == 2 && string.Compare(str2, str1) == -1)
              flag2 = true;
          }
          if (flag2)
          {
            flag1 = true;
            let mut width: i32 = self.Width;
            for (let mut index2: i32 = 0; index2 <= width; index2 += 1)
            {
              str3: String = self.Data[index1, index2];
              let mut num6: i32 = self.TempBmp[index1, index2];
              str4: String = self.TempDesc[index1, index2];
              self.Data[index1, index2] = self.Data[index1 + 1, index2];
              self.TempBmp[index1, index2] = self.TempBmp[index1 + 1, index2];
              self.TempDesc[index1, index2] = self.TempDesc[index1 + 1, index2];
              self.Data[index1 + 1, index2] = str3;
              self.TempBmp[index1 + 1, index2] = num6;
              self.TempDesc[index1 + 1, index2] = str4;
            }
          }
        }
      }
    }

    pub fn RemoveRow(row: i32)
    {
      object[,] objArray = new object[self.Length + 1, self.Width + 1];
      let mut length1: i32 = self.Length;
      for (let mut index1: i32 = 0; index1 <= length1; index1 += 1)
      {
        let mut width: i32 = self.Width;
        for (let mut index2: i32 = 0; index2 <= width; index2 += 1)
          objArray[index1, index2] =  self.Data[index1, index2];
      }
      if (row < self.Length)
      {
        let mut num1: i32 = row;
        let mut num2: i32 = self.Length - 1;
        for (let mut index3: i32 = num1; index3 <= num2; index3 += 1)
        {
          let mut width: i32 = self.Width;
          for (let mut index4: i32 = 0; index4 <= width; index4 += 1)
            objArray[index3, index4] =  self.Data[index3 + 1, index4];
        }
      }
      --self.Length;
      self.Data = new string[self.Length + 1, self.Width + 1];
      let mut length2: i32 = self.Length;
      for (let mut index5: i32 = 0; index5 <= length2; index5 += 1)
      {
        let mut width: i32 = self.Width;
        for (let mut index6: i32 = 0; index6 <= width; index6 += 1)
          self.Data[index5, index6] = Conversions.ToString(objArray[index5, index6]);
      }
    }

    pub fn RemoveMultipleRow(bool[] flaggy)
    {
      object[,] objArray = new object[self.Length + 1, self.Width + 1];
      let mut index1: i32 = -1;
      let mut num: i32 = 0;
      let mut length1: i32 = self.Length;
      for (let mut index2: i32 = 0; index2 <= length1; index2 += 1)
      {
        if (!flaggy[index2])
        {
          index1 += 1;
          let mut width: i32 = self.Width;
          for (let mut index3: i32 = 0; index3 <= width; index3 += 1)
            objArray[index1, index3] =  self.Data[index2, index3];
        }
        else
          num += 1;
      }
      self.Length -= num;
      if (self.Length > -1)
      {
        self.Data = new string[self.Length + 1, self.Width + 1];
        self.TempBmp = new int[self.Length + 1, self.Width + 1];
        self.TempDesc = new string[self.Length + 1, self.Width + 1];
        let mut length2: i32 = self.Length;
        for (let mut index4: i32 = 0; index4 <= length2; index4 += 1)
        {
          let mut width: i32 = self.Width;
          for (let mut index5: i32 = 0; index5 <= width; index5 += 1)
            self.Data[index4, index5] = Conversions.ToString(objArray[index4, index5]);
        }
      }
      else
      {
        self.Data = new string[1, self.Width + 1];
        self.TempBmp = new int[1, self.Width + 1];
        self.TempDesc = new string[1, self.Width + 1];
        self.Length = -1;
      }
    }

    pub fn RemoveCol(col: i32)
    {
      object[,] objArray = new object[self.Length + 1, self.Width + 1];
      let mut length1: i32 = self.Length;
      for (let mut index1: i32 = 0; index1 <= length1; index1 += 1)
      {
        let mut width: i32 = self.Width;
        for (let mut index2: i32 = 0; index2 <= width; index2 += 1)
          objArray[index1, index2] =  self.Data[index1, index2];
      }
      if (col < self.Width)
      {
        let mut length2: i32 = self.Length;
        for (let mut index3: i32 = 0; index3 <= length2; index3 += 1)
        {
          let mut num1: i32 = col;
          let mut num2: i32 = self.Width - 1;
          for (let mut index4: i32 = num1; index4 <= num2; index4 += 1)
            objArray[index3, index4] =  self.Data[index3, index4 + 1];
        }
        let mut num3: i32 = col;
        let mut num4: i32 = self.Width - 1;
        for (let mut index: i32 = num3; index <= num4; index += 1)
        {
          self.ColumnName[index] = self.ColumnName[index + 1];
          self.logEnabled[index] = self.logEnabled[index + 1];
          self.ColSort[index] = self.ColSort[index + 1];
          self.ColWidth[index] = self.ColWidth[index + 1];
          self.ColValueType[index] = self.ColValueType[index + 1];
          self.LookUpCol[index] = self.LookUpCol[index + 1];
        }
      }
      --self.Width;
      self.ColumnName = (string[]) Utils.CopyArray((Array) self.ColumnName, (Array) new string[self.Width + 1]);
      self.logEnabled = (bool[]) Utils.CopyArray((Array) self.logEnabled, (Array) new bool[self.Width + 1]);
      self.ColSort = (int[]) Utils.CopyArray((Array) self.ColSort, (Array) new int[self.Width + 1]);
      self.ColWidth = (int[]) Utils.CopyArray((Array) self.ColWidth, (Array) new int[self.Width + 1]);
      self.LookUpCol = (int[]) Utils.CopyArray((Array) self.LookUpCol, (Array) new int[self.Width + 1]);
      self.SSID = (int[]) Utils.CopyArray((Array) self.SSID, (Array) new int[self.Width + 1]);
      self.ColValueType = (NewEnums.LibVarValueType[]) Utils.CopyArray((Array) self.ColValueType, (Array) new NewEnums.LibVarValueType[self.Width + 1]);
      self.Data = new string[self.Length + 1, self.Width + 1];
      let mut length3: i32 = self.Length;
      for (let mut index5: i32 = 0; index5 <= length3; index5 += 1)
      {
        let mut width: i32 = self.Width;
        for (let mut index6: i32 = 0; index6 <= width; index6 += 1)
          self.Data[index5, index6] = Conversions.ToString(objArray[index5, index6]);
      }
    }

    pub fn AddRow(row: i32)
    {
      if (row >= self.Length)
      {
        self.AddRowFast(row);
      }
      else
      {
        object[,] objArray1 = new object[self.Length + 1 + 1, self.Width + 1];
        object[,] objArray2 = new object[self.Length + 1 + 1, self.Width + 1];
        object[,] objArray3 = new object[self.Length + 1 + 1, self.Width + 1];
        let mut length1: i32 = self.Length;
        for (let mut index1: i32 = 0; index1 <= length1; index1 += 1)
        {
          let mut width: i32 = self.Width;
          for (let mut index2: i32 = 0; index2 <= width; index2 += 1)
          {
            objArray1[index1, index2] =  self.Data[index1, index2];
            objArray2[index1, index2] =  self.TempBmp[index1, index2];
            objArray3[index1, index2] =  self.TempDesc[index1, index2];
          }
        }
        if (row < self.Length)
        {
          this += 1.Length;
          self.Data = new string[self.Length + 1, self.Width + 1];
          self.TempBmp = new int[self.Length + 1, self.Width + 1];
          self.TempDesc = new string[self.Length + 1, self.Width + 1];
          let mut num1: i32 = self.Length - 1;
          for (let mut index3: i32 = 0; index3 <= num1; index3 += 1)
          {
            let mut width: i32 = self.Width;
            for (let mut index4: i32 = 0; index4 <= width; index4 += 1)
            {
              self.Data[index3, index4] = Conversions.ToString(objArray1[index3, index4]);
              self.TempBmp[index3, index4] = Conversions.ToInteger(objArray2[index3, index4]);
              self.TempDesc[index3, index4] = Conversions.ToString(objArray3[index3, index4]);
            }
          }
          let mut length2: i32 = self.Length;
          let mut num2: i32 = row + 2;
          for (let mut index5: i32 = length2; index5 >= num2; index5 += -1)
          {
            let mut width: i32 = self.Width;
            for (let mut index6: i32 = 0; index6 <= width; index6 += 1)
            {
              self.Data[index5, index6] = Conversions.ToString(objArray1[index5 - 1, index6]);
              self.TempBmp[index5, index6] = Conversions.ToInteger(objArray2[index5 - 1, index6]);
              self.TempDesc[index5, index6] = Conversions.ToString(objArray3[index5 - 1, index6]);
            }
          }
          let mut width1: i32 = self.Width;
          for (let mut index: i32 = 0; index <= width1; index += 1)
          {
            self.Data[row + 1, index] = "";
            self.TempBmp[row + 1, index] = 0;
            self.TempDesc[row + 1, index] = "";
          }
        }
        else
        {
          this += 1.Length;
          self.Data = new string[self.Length + 1, self.Width + 1];
          self.TempBmp = new int[self.Length + 1, self.Width + 1];
          self.TempDesc = new string[self.Length + 1, self.Width + 1];
          let mut num: i32 = self.Length - 1;
          for (let mut index7: i32 = 0; index7 <= num; index7 += 1)
          {
            let mut width: i32 = self.Width;
            for (let mut index8: i32 = 0; index8 <= width; index8 += 1)
            {
              self.Data[index7, index8] = Conversions.ToString(objArray1[index7, index8]);
              self.TempBmp[index7, index8] = Conversions.ToInteger(objArray2[index7, index8]);
              self.TempDesc[index7, index8] = Conversions.ToString(objArray3[index7, index8]);
            }
          }
          let mut width2: i32 = self.Width;
          for (let mut index: i32 = 0; index <= width2; index += 1)
          {
            self.Data[self.Length, index] = "";
            self.TempBmp[row + 1, index] = 0;
            self.TempDesc[row + 1, index] = "";
          }
        }
        let mut upperBound1: i32 = self.Data.GetUpperBound(0);
        for (let mut index9: i32 = 0; index9 <= upperBound1; index9 += 1)
        {
          let mut upperBound2: i32 = self.Data.GetUpperBound(1);
          for (let mut index10: i32 = 0; index10 <= upperBound2; index10 += 1)
          {
            if (Information.IsNothing( self.Data[index9, index10]))
              self.Data[index9, index10] = "";
          }
        }
      }
    }

    pub fn AddRowFast(row: i32)
    {
      if (row < self.Length)
      {
        let mut num1: i32 =  Interaction.MsgBox( "Error, can only add at end of list with AddRowFast");
      }
      else
      {
        if (self.Length >= self.Data.GetUpperBound(0) | self.Data.GetUpperBound(1) < self.Width)
        {
          object[,] objArray1 = new object[self.Length + 1 + 1, self.Width + 1];
          object[,] objArray2 = new object[self.Length + 1 + 1, self.Width + 1];
          object[,] objArray3 = new object[self.Length + 1 + 1, self.Width + 1];
          let mut length: i32 = self.Length;
          for (let mut index1: i32 = 0; index1 <= length; index1 += 1)
          {
            let mut width: i32 = self.Width;
            for (let mut index2: i32 = 0; index2 <= width; index2 += 1)
            {
              objArray1[index1, index2] =  self.Data[index1, index2];
              objArray2[index1, index2] =  self.TempBmp[index1, index2];
              objArray3[index1, index2] =  self.TempDesc[index1, index2];
            }
          }
          this += 1.Length;
          if (self.Length >= 100000)
          {
            self.Data = new string[self.Length + 100000 + 1, self.Width + 1];
            self.TempBmp = new int[self.Length + 100000 + 1, self.Width + 1];
            self.TempDesc = new string[self.Length + 100000 + 1, self.Width + 1];
          }
          else if (self.Length >= 30000)
          {
            self.Data = new string[self.Length + 30000 + 1, self.Width + 1];
            self.TempBmp = new int[self.Length + 30000 + 1, self.Width + 1];
            self.TempDesc = new string[self.Length + 30000 + 1, self.Width + 1];
          }
          else if (self.Length >= 10000)
          {
            self.Data = new string[self.Length + 10000 + 1, self.Width + 1];
            self.TempBmp = new int[self.Length + 10000 + 1, self.Width + 1];
            self.TempDesc = new string[self.Length + 10000 + 1, self.Width + 1];
          }
          else if (self.Length >= 1000)
          {
            self.Data = new string[self.Length + 1000 + 1, self.Width + 1];
            self.TempBmp = new int[self.Length + 1000 + 1, self.Width + 1];
            self.TempDesc = new string[self.Length + 1000 + 1, self.Width + 1];
          }
          else
          {
            self.Data = new string[self.Length + 100 + 1, self.Width + 1];
            self.TempBmp = new int[self.Length + 100 + 1, self.Width + 1];
            self.TempDesc = new string[self.Length + 100 + 1, self.Width + 1];
          }
          let mut num2: i32 = self.Length - 1;
          for (let mut index3: i32 = 0; index3 <= num2; index3 += 1)
          {
            let mut width: i32 = self.Width;
            for (let mut index4: i32 = 0; index4 <= width; index4 += 1)
            {
              self.Data[index3, index4] = Conversions.ToString(objArray1[index3, index4]);
              self.TempBmp[index3, index4] = Conversions.ToInteger(objArray2[index3, index4]);
              self.TempDesc[index3, index4] = Conversions.ToString(objArray3[index3, index4]);
            }
          }
        }
        else
          this += 1.Length;
        let mut width1: i32 = self.Width;
        for (let mut index: i32 = 0; index <= width1; index += 1)
        {
          if (self.TempBmp.Length < self.Data.Length)
          {
            self.TempBmp = new int[self.Length + 1, self.Width + 1];
            self.TempDesc = new string[self.Length + 1, self.Width + 1];
          }
          self.Data[self.Length, index] = "";
          self.TempBmp[self.Length, index] = 0;
          self.TempDesc[self.Length, index] = "";
        }
      }
    }

    pub fn AddCol(col: i32, tname: String = "New Col")
    {
      self.ColumnName = (string[]) Utils.CopyArray((Array) self.ColumnName, (Array) new string[self.Width + 1 + 1]);
      self.logEnabled = (bool[]) Utils.CopyArray((Array) self.logEnabled, (Array) new bool[self.Width + 1 + 1]);
      self.ColSort = (int[]) Utils.CopyArray((Array) self.ColSort, (Array) new int[self.Width + 1 + 1]);
      self.ColWidth = (int[]) Utils.CopyArray((Array) self.ColWidth, (Array) new int[self.Width + 1 + 1]);
      self.LookUpCol = (int[]) Utils.CopyArray((Array) self.LookUpCol, (Array) new int[self.Width + 1 + 1]);
      self.SSID = (int[]) Utils.CopyArray((Array) self.SSID, (Array) new int[self.Width + 1 + 1]);
      self.ColValueType = (NewEnums.LibVarValueType[]) Utils.CopyArray((Array) self.ColValueType, (Array) new NewEnums.LibVarValueType[self.Width + 1 + 1]);
      self.TempColumBmp = (string[]) Utils.CopyArray((Array) self.TempColumBmp, (Array) new string[self.Width + 1 + 1]);
      object[,] objArray = new object[self.Length + 1 + 1, self.Width + 1];
      let mut length1: i32 = self.Length;
      for (let mut index1: i32 = 0; index1 <= length1; index1 += 1)
      {
        let mut width: i32 = self.Width;
        for (let mut index2: i32 = 0; index2 <= width; index2 += 1)
          objArray[index1, index2] =  self.Data[index1, index2];
      }
      if (col + 1 < self.Width)
      {
        this += 1.Width;
        if (self.Length > -1)
          self.Data = (string[,]) Utils.CopyArray((Array) self.Data, (Array) new string[self.Data.GetUpperBound(0) + 1, self.Width + 1]);
        if (self.Length > -1)
          self.TempBmp = (int[,]) Utils.CopyArray((Array) self.TempBmp, (Array) new int[self.TempBmp.GetUpperBound(0) + 1, self.Width + 1]);
        if (self.Length > -1)
          self.TempDesc = (string[,]) Utils.CopyArray((Array) self.TempDesc, (Array) new string[self.TempDesc.GetUpperBound(0) + 1, self.Width + 1]);
        let mut length2: i32 = self.Length;
        for (let mut index3: i32 = 0; index3 <= length2; index3 += 1)
        {
          let mut width: i32 = self.Width;
          let mut num: i32 = col + 2;
          for (let mut index4: i32 = width; index4 >= num; index4 += -1)
            self.Data[index3, index4] = Conversions.ToString(objArray[index3, index4 - 1]);
        }
        let mut width1: i32 = self.Width;
        let mut num1: i32 = col + 2;
        for (let mut index: i32 = width1; index >= num1; index += -1)
        {
          self.ColumnName[index] = self.ColumnName[index - 1];
          self.logEnabled[index] = self.logEnabled[index - 1];
          self.ColSort[index] = self.ColSort[index - 1];
          self.ColWidth[index] = self.ColWidth[index - 1];
          self.ColValueType[index] = self.ColValueType[index - 1];
        }
        let mut length3: i32 = self.Length;
        for (let mut index: i32 = 0; index <= length3; index += 1)
          self.Data[index, col + 1] = "";
        self.ColumnName[col + 1] = tname;
        self.logEnabled[col + 1] = false;
        self.ColValueType[col + 1] = NewEnums.LibVarValueType.Text;
      }
      else
      {
        this += 1.Width;
        if (self.Length > -1)
          self.Data = (string[,]) Utils.CopyArray((Array) self.Data, (Array) new string[self.Data.GetUpperBound(0) + 1, self.Width + 1]);
        if (self.Length > -1)
          self.TempBmp = (int[,]) Utils.CopyArray((Array) self.TempBmp, (Array) new int[self.TempBmp.GetUpperBound(0) + 1, self.Width + 1]);
        if (self.Length > -1)
          self.TempDesc = (string[,]) Utils.CopyArray((Array) self.TempDesc, (Array) new string[self.TempDesc.GetUpperBound(0) + 1, self.Width + 1]);
        let mut length4: i32 = self.Length;
        for (let mut index: i32 = 0; index <= length4; index += 1)
          self.Data[index, self.Width] = "";
        self.ColumnName[self.Width] = tname;
        self.logEnabled[self.Width] = false;
        self.ColValueType[self.Width] = NewEnums.LibVarValueType.Text;
      }
      let mut upperBound1: i32 = self.Data.GetUpperBound(0);
      for (let mut index5: i32 = 0; index5 <= upperBound1; index5 += 1)
      {
        let mut upperBound2: i32 = self.Data.GetUpperBound(1);
        for (let mut index6: i32 = 0; index6 <= upperBound2; index6 += 1)
        {
          if (Information.IsNothing( self.Data[index5, index6]))
            self.Data[index5, index6] = "";
        }
      }
    }
  }
}
