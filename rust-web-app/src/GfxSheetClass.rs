// Decompiled with JetBrains decompiler
// Type: WindowsApplication1.GfxSheetClass
// Assembly: WindowsApplication1, Version=1.0.8020.28903, Culture=neutral, PublicKeyToken=null
// MVID: F52869E5-0850-48AD-BBBE-68E7A4900AFE
// Assembly location: C:\Program Files (x86)\Steam\steamapps\common\Shadow Empire\ShadowEmpire.exe

// usingMicrosoft.VisualBasic;
// usingSystem;
// usingSystem.Drawing;
// usingSystem.IO;

namespace WindowsApplication1
{
  pub class GfxSheetClass
  {
    pub Counter: i32;
    pub DirName: String;
    pub Bmp: Bitmap;
    pub Name: Vec<String>;
    pub Rectangle[] Rect;

    pub GfxSheetClass()
    {
      this.Name = new string[2];
      this.Rect = Rectangle::new[2];
      this.Counter = 0;
      this.Bmp = (Bitmap) null;
    }

    pub fn Load(tDirName: String)
    {
      this.DirName = tDirName;
      StreamReader streamReader = File.OpenText(DrawMod.TGame.AppPath + "graphics/" + this.DirName + "/sheet.txt");
      this.Counter =  Math.Round(Conversion.Val(streamReader.ReadLine()));
      this.Name = new string[this.Counter + 1];
      this.Rect = Rectangle::new[this.Counter + 1];
      let mut counter: i32 =  this.Counter;
      for (let mut index: i32 =  1; index <= counter; index += 1)
      {
        this.Name[index] = streamReader.ReadLine();
        this.Rect[index].X =  Math.Round(Conversion.Val(streamReader.ReadLine()));
        this.Rect[index].Y =  Math.Round(Conversion.Val(streamReader.ReadLine()));
        this.Rect[index].Width =  Math.Round(Conversion.Val(streamReader.ReadLine()));
        this.Rect[index].Height =  Math.Round(Conversion.Val(streamReader.ReadLine()));
      }
      streamReader.Close();
      this.Bmp = new Bitmap(DrawMod.TGame.AppPath + "graphics/" + this.DirName + "/sheet.png");
      this.Bmp.SetResolution( DrawMod.DPIx,  DrawMod.DPIy);
    }
  }
}
