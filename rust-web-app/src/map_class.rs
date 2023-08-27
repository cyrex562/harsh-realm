// Decompiled with JetBrains decompiler
#![allow(non_snake_case)]
// Type: WindowsApplication1.MapClass
// Assembly: WindowsApplication1, Version=1.0.8020.28903, Culture=neutral, PublicKeyToken=null
// MVID: F52869E5-0850-48AD-BBBE-68E7A4900AFE
// Assembly location: C:\Program Files (x86)\Steam\steamapps\common\Shadow Empire\ShadowEmpire.exe

// usingMicrosoft.VisualBasic;
// usingMicrosoft.VisualBasic.CompilerServices;
// usingSystem;
// usingSystem.IO;
// usingSystem.Runtime.Serialization;
// usingSystem.Windows.Forms;

// namespace WindowsApplication1
// {
//   [Serializable]
//   pub class MapClass : ISerializable
pub struct MapClass
{
    pub Name: String,
    pub MapWidth: i32,
    pub MapHeight: i32,
    pub HexObj: Vec<HexClass>,
    pub MapLoop: bool,
    pub CanSee: bool,
    pub TempCanSee: bool,


  }
// }

impl MapClass {
     pub fn GetObjectData(SerializationInfo info, StreamingContext context)
    {
      GC.Collect();
      Application.DoEvents();
      info.AddValue("Name",  this.Name);
      info.AddValue("MapWidth", this.MapWidth);
      info.AddValue("MapHeight", this.MapHeight);
      let mut num1: i32 =  164 * ((this.MapWidth + 1) * (this.MapHeight + 1)) + 24 * ((this.MapWidth + 1) * (this.MapHeight + 1)) + 4 * ((this.MapWidth + 1) * (this.MapHeight + 1));
      let mut num2: i32 =  4;
      let mut num3: i32 =  num1 + 112 * ((this.MapWidth + 1) * (this.MapHeight + 1));
      let mut mapWidth1: i32 =  this.MapWidth;
      index1: i32;
      index2: i32;
      for (index1 = 0; index1 <= mapWidth1; index1 += 1)
      {
        let mut mapHeight: i32 =  this.MapHeight;
        for (index2 = 0; index2 <= mapHeight; index2 += 1)
        {
          if (this.HexObj[index1, index2].RegimeCount > -1)
            num3 += 56 * (1 + this.HexObj[index1, index2].RegimeCount);
          if (this.HexObj[index1, index2].RegimeFullCount > -1)
            num3 += 8 * (1 + this.HexObj[index1, index2].RegimeFullCount);
          if (this.HexObj[index1, index2].ConnectionCount > -1)
            num3 += 12 * (1 + this.HexObj[index1, index2].ConnectionCount);
          if (this.HexObj[index1, index2].UnitCounter > -1)
            num3 += 4 * (1 + this.HexObj[index1, index2].UnitCounter);
          if (Information.IsNothing( this.HexObj[index1, index2].Name))
            this.HexObj[index1, index2].Name = "";
          if (Information.IsNothing( this.HexObj[index1, index2].LabelText1))
            this.HexObj[index1, index2].LabelText1 = "";
          if (Information.IsNothing( this.HexObj[index1, index2].LabelText2))
            this.HexObj[index1, index2].LabelText2 = "";
          if (Information.IsNothing( this.HexObj[index1, index2].SmallLabel))
            this.HexObj[index1, index2].SmallLabel = "";
          num3 = num3 + (10 + this.HexObj[index1, index2].SmallLabel.Length * num2) + (10 + this.HexObj[index1, index2].Name.Length * num2) + (10 + this.HexObj[index1, index2].LabelText1.Length * num2) + (10 + this.HexObj[index1, index2].LabelText2.Length * num2);
          if (this.HexObj[index1, index2].HexLibVarCounter > -1)
            num3 += 8 * (1 + this.HexObj[index1, index2].HexLibVarCounter);
          if (DrawMod.TGame.Data.Product >= 5)
            num3 += 4;
          if (DrawMod.TGame.Data.Product >= 6)
            num3 = num3 + 12 + 24 + 4;
          if (DrawMod.TGame.Data.Product == 7)
            num3 = num3 + 180 + 36;
        }
      }
      let mut num4: i32 =  num3;
      byte[] numArray1 = new byte[num4 + 1];
      MemoryStream output = new MemoryStream(numArray1);
      BinaryWriter bw = new BinaryWriter((Stream) output);
      try
      {
        let mut mapWidth2: i32 =  this.MapWidth;
        for (index1 = 0; index1 <= mapWidth2; index1 += 1)
        {
          let mut mapHeight: i32 =  this.MapHeight;
          for (index2 = 0; index2 <= mapHeight; index2 += 1)
          {
            this.HexObj[index1, index2].StreamWrite( bw);
            bw = bw;
          }
          bw.Flush();
          output.Flush();
        }
      }
      catch (Exception ex)
      {
        ProjectData.SetProjectError(ex);
        Exception exception = ex;
        let mut num5: i32 =   Interaction.MsgBox( ("Error in hex data compression. Current size: " + bw.BaseStream.Position.ToString() + " , SizeDimmed: " + num4.ToString() + "..  ix: " + index1.ToString() + "/" + this.MapWidth.ToString() + " iy: " + index2.ToString() + "/" + this.MapHeight.ToString() + " .. actual error message: " + exception.Message));
        ProjectData.ClearProjectError();
      }
      bw.Flush();
      output.Flush();
      byte[] numArray2 = (byte[]) Utils.CopyArray((Array) numArray1, (Array) new byte[ (output.Position + 1L) + 1]);
      info.AddValue("DataSize", numArray2.GetUpperBound(0));
      info.AddValue("Data",  numArray2);
      bw.Close();
      output.Close();
      info.AddValue("MapLoop", this.MapLoop);
      info.AddValue("CanSee", this.CanSee);
    }

    protected MapClass(SerializationInfo info, StreamingContext context)
    {
      this.HexObj = new HexClass[1, 1];
      GC.Collect();
      Application.DoEvents();
      this.Name = info.GetString(nameof (Name));
      this.MapWidth = info.GetInt32(nameof (MapWidth));
      this.MapHeight = info.GetInt32(nameof (MapHeight));
      this.HexObj = new HexClass[this.MapWidth + 1, this.MapHeight + 1];
      try
      {
        this.HexObj = (HexClass[,]) info.GetValue(nameof (HexObj), this.HexObj.GetType());
      }
      catch (Exception ex)
      {
        ProjectData.SetProjectError(ex);
        byte[] numArray1 = new byte[info.GetInt32("DataSize") + 1];
        Stream input = (Stream) new MemoryStream((byte[]) info.GetValue("Data", numArray1.GetType()));
        BinaryReader br = new BinaryReader(input);
        let mut mapWidth: i32 =  this.MapWidth;
        for (let mut index1: i32 =  0; index1 <= mapWidth; index1 += 1)
        {
          let mut mapHeight: i32 =  this.MapHeight;
          for (let mut index2: i32 =  0; index2 <= mapHeight; index2 += 1)
          {
            this.HexObj[index1, index2] = new HexClass(0, 0, 0);
            this.HexObj[index1, index2].StreamRead( br);
          }
        }
        byte[] numArray2 = new byte[1];
        br.Close();
        input.Close();
        ProjectData.ClearProjectError();
      }
      this.MapLoop = info.GetBoolean(nameof (MapLoop));
      try
      {
        this.CanSee = info.GetBoolean(nameof (CanSee));
      }
      catch (Exception ex)
      {
        ProjectData.SetProjectError(ex);
        this.CanSee = false;
        ProjectData.ClearProjectError();
      }
    }

    pub MapClass(hardcoded: i32, regimecount: i32, w: i32, h: i32)
    {
      this.HexObj = new HexClass[1, 1];
      this.Name = "New Extra Map";
      this.MapWidth = w;
      this.MapHeight = h;
      this.MapLoop = false;
      this.CanSee = false;
      this.HexObj = new HexClass[w + 1, h + 1];
      let mut num1: i32 =  w;
      for (let mut index1: i32 =  0; index1 <= num1; index1 += 1)
      {
        let mut num2: i32 =  h;
        for (let mut index2: i32 =  0; index2 <= num2; index2 += 1)
        {
          this.HexObj[index1, index2] = new HexClass(0, regimecount, regimecount);
          this.HexObj[index1, index2].Location2 = -1;
        }
      }
    }
}
