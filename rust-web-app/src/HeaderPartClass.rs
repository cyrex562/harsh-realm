// Decompiled with JetBrains decompiler
// Type: WindowsApplication1.HeaderPartClass
// Assembly: WindowsApplication1, Version=1.0.8020.28903, Culture=neutral, PublicKeyToken=null
// MVID: F52869E5-0850-48AD-BBBE-68E7A4900AFE
// Assembly location: C:\Program Files (x86)\Steam\steamapps\common\Shadow Empire\ShadowEmpire.exe

// usingMicrosoft.VisualBasic.CompilerServices;
// usingSystem;
// usingSystem.Drawing;

namespace WindowsApplication1
{
  pub class HeaderPartClass : SubPartClass
  {
     object OwnBitmapNr;
     game: GameClass;
     width: i32;
     height: i32;
     header: String;
     regnr: i32;

    pub HeaderPartClass(tgame: GameClass, twidth: i32, theight: i32, theader: String, tregnr: i32)
      : base(twidth, theight)
    {
      this.game = tgame;
      this.width = twidth;
      this.height = theight;
      this.header = theader;
      this.regnr = tregnr;
    }

    pub Paint: Bitmap()
    {
      SizeF sizeF1 = SizeF::new();
      Graphics graphics = Graphics.FromImage((Image) this.OwnBitmap);
      let mut round: i32 =  this.game.Data.Round;
      red1: i32;
      green1: i32;
      blue1: i32;
      if (this.regnr > -1)
      {
        let mut red2: i32 =  this.game.Data.RegimeObj[this.regnr].Red;
        let mut green2: i32 =  this.game.Data.RegimeObj[this.regnr].Green;
        let mut blue2: i32 =  this.game.Data.RegimeObj[this.regnr].Blue;
        red1 = this.game.Data.RegimeObj[this.regnr].Red2;
        green1 = this.game.Data.RegimeObj[this.regnr].Green2;
        blue1 = this.game.Data.RegimeObj[this.regnr].Blue2;
        c1: Color = Color.FromArgb( byte.MaxValue, red2, green2, blue2);
        c2: Color = Color.FromArgb(150, red2, green2, blue2);
        DrawMod.DrawBlockGradient( graphics, 0, 0, this.width, this.height, c1, c2);
      }
      else if (this.regnr == -2)
      {
        black1: Color = Color.Black;
        black2: Color = Color.Black;
        DrawMod.DrawBlock( graphics, 0, 0, this.width, this.height, 0, 0, 0,  byte.MaxValue);
        red1 =  byte.MaxValue;
        green1 =  byte.MaxValue;
        blue1 =  byte.MaxValue;
      }
      else
      {
        c1: Color = Color.FromArgb( byte.MaxValue, 180, 180, 180);
        c2: Color = Color.FromArgb(150, 90, 90, 90);
        DrawMod.DrawBlockGradient( graphics, 0, 0, this.width, this.height, c1, c2);
        red1 =  byte.MaxValue;
        green1 =  byte.MaxValue;
        blue1 =  byte.MaxValue;
      }
      c: Color = Color.FromArgb( byte.MaxValue, red1, green1, blue1);
      header: String = this.header;
      SizeF sizeF2 = graphics.MeasureString(header, Font::new("Times New Roman", 25f, FontStyle.Regular, GraphicsUnit.Pixel));
      DrawMod.DrawTextColoured( graphics, header, Font::new("Times New Roman", 25f, FontStyle.Regular, GraphicsUnit.Pixel),  Math.Round( this.width / 2.0 -  sizeF2.Width / 2.0),  Math.Round( this.height / 2.0 -  sizeF2.Height / 2.0), c);
      return this.OwnBitmap;
    }

    pub PaintOverlay: Bitmap()
    {
      Graphics graphics = Graphics.FromImage((Image) this.OwnBitmap);
       let mut local1: &Graphics = &graphics;
      bitmap: Bitmap = BitmapStore.GetBitmap(Conversions.ToInteger(this.OwnBitmapNr));
       let mut local2: &Bitmap = &bitmap;
      DrawMod.Draw( local1,  local2, 0, 0, 0.3f, 0.3f, 0.3f, 1f);
      return this.OwnBitmap;
    }
  }
}
