// Decompiled with JetBrains decompiler
// Type: WindowsApplication1.CreditsInfoWindowClass
// Assembly: WindowsApplication1, Version=1.0.8020.28903, Culture=neutral, PublicKeyToken=null
// MVID: F52869E5-0850-48AD-BBBE-68E7A4900AFE
// Assembly location: C:\Program Files (x86)\Steam\steamapps\common\Shadow Empire\ShadowEmpire.exe

// usingMicrosoft.VisualBasic.CompilerServices;
// usingSystem;
// usingSystem.Drawing;
// usingSystem.Drawing.Drawing2D;

namespace WindowsApplication1
{
  pub class CreditsInfoWindowClass : WindowClass
  {
     okid: i32;
     cancelid: i32;
     oktextid: i32;
     Pic1Id: i32;
     TAid: i32;
     Ta2id: i32;
     His: i32;
     Card: i32;
     Unr: i32;
     tx: i32;
     ty: i32;
     tr: i32;
     tw: i32;
     th: i32;

    pub CreditsInfoWindowClass(ref tGame: GameClass)
      : base(ref tGame, 880, 680, 8)
    {
      this.View();
      this.tr =  byte.MaxValue;
      this.tx = 0;
      this.ty = 0;
    }

    pub fn HandleToolTip(x: i32, y: i32)
    {
      base.HandleToolTip(x, y);
      if (this.SubPartCounter > -1)
      {
        let mut subPartCounter: i32 =  this.SubPartCounter;
        for (let mut index: i32 =  0; index <= subPartCounter; index += 1)
        {
          if (x > this.SubPartX[index] & x < this.SubPartX[index] + this.SubPartW[index] && y > this.SubPartY[index] & y < this.SubPartY[index] + this.SubPartH[index])
          {
            this.SubPartList[index].DescriptInfo(x - this.SubPartX[index], y - this.SubPartY[index]);
            if (Operators.CompareString(this.SubPartList[index].Descript, "", false) > 0)
            {
              this.game.EditObj.TipButton = true;
              this.game.EditObj.TipTitle = "";
              this.game.EditObj.TipText = this.SubPartList[index].Descript;
              return;
            }
          }
        }
      }
      let mut mouseCounter: i32 =  this.MouseCounter;
      for (let mut index: i32 =  0; index <= mouseCounter; index += 1)
      {
        if (x > this.MouseRect[index].X & x < this.MouseRect[index].X + this.MouseRect[index].Width && y > this.MouseRect[index].Y & y < this.MouseRect[index].Y + this.MouseRect[index].Height)
        {
          if (this.MouseData[index] > 0)
            this.game.EditObj.TipButton = true;
          this.game.EditObj.TipTitle = this.MouseTitle[index];
          this.game.EditObj.TipText = this.MouseText[index];
          break;
        }
      }
    }

    pub fn View()
    {
      if (this.cancelid > 0)
      {
        this.RemoveSubPart(this.cancelid);
        this.cancelid = 0;
      }
      if (this.TAid > 0)
        this.RemoveSubPart(this.TAid);
      if (this.Ta2id > 0)
        this.RemoveSubPart(this.Ta2id);
      this.ClearMouse();
      this.NewBackGroundAndClearAll(880, 680, -1);
      Graphics g = Graphics.FromImage((Image) this.OwnBitmap);
      g.SmoothingMode = SmoothingMode.HighQuality;
      g.InterpolationMode = InterpolationMode.HighQualityBicubic;
      DrawMod.DrawMessFrame(ref this.OwnBitmap, ref g, 0, 0, 880, 680);
      this.BackBitmap = (Bitmap) this.OwnBitmap.Clone();
      tText1: String = "Designed by Victor Reijkersz" + "\r\n" + "\r\n" + "PROGRAMMING, ARTIFICIAL INTELLIGENCE" + "\r\n" + "Victor Reijkersz" + "\r\n" + "\r\n" + "ARTWORK" + "\r\n" + "Mike Tenebrae, Bernd Brossig, Victor Reijkersz, Frederic Genot and others" + "\r\n" + "\r\n" + "WRITING" + "\r\n" + "Victor Reijkersz" + "\r\n" + "\r\n" + "MUSIC" + "\r\n" + "Tempest for an Angel" + "\r\n" + "\r\n" + "SOUND EFFECTS" + "\r\n" + "Tempest for an Angel, PMSFX, Benjamin D. Halford and others" + "\r\n" + "\r\n" + "SOLDIER VOICES" + "\r\n" + "Georgina Donnalley, CoderVA, Nylithia, Rachel Bell and others" + "\r\n" + "\r\n" + "MANUAL" + "\r\n" + "Victor Reijkersz" + "\r\n" + "\r\n" + "MANUAL POST-RELEASE PROOFREADING" + "\r\n" + "Tom Try" + "\r\n" + "\r\n" + "TESTING AND FEEDBACK" + "\r\n" + "Bavarian Kid, Clux, Soar_Slitherine, Nachtjager, DTurtle, Malevolence, Culthrasa, Jim Winsor, Eric Heiser, Cassini, Benjamin Graham, Andy Brown, Cablenexus, Zakblood, Verde321, Cablenexus, Jnpoint, Gwgardner, JHeinlen, Aristos, Shards, DasTactic, Jakval96, TortugaPower, Pocus, OutOf8, ArditiDagger, Maheb, Dan1911, Wodin, Tufkal2, David Arenz, C.Charles Dunlap, Harry Chrismas, Scott Jackson and many many others!" + "\r\n" + "\r\n";
      tText2: String = "Published by Matrix Games" + "\r\n" + "\r\n" + "CEO" + "\r\n" + "Iain McNeil" + "\r\n" + "\r\n" + "CFO" + "\r\n" + "JD McNeil" + "\r\n" + "\r\n" + "SENIOR PRODUCER" + "\r\n" + "David Sharrock" + "\r\n" + "\r\n" + "PRODUCERS" + "\r\n" + "Tamas Kiss, Bart Schouten, Neil McKenna, Erik Rutins" + "\r\n" + "\r\n" + "TECHNICAL DIRECTOR" + "\r\n" + "Philip Veale" + "\r\n" + "\r\n" + "CREATIVE DIRECTOR" + "\r\n" + "Richard Evans" + "\r\n" + "\r\n" + "MARKETING DIRECTOR " + "\r\n" + "Marco Minoli" + "\r\n" + "\r\n" + "PRODUCT MANAGER" + "\r\n" + "Roberta Migliori, Alberto Casulini, Daniele Meneghini" + "\r\n" + "\r\n" + "SOCIAL MEDIA MANAGER" + "\r\n" + "Bruno Bontempo" + "\r\n" + "\r\n" + "MEDIA RELATIONS" + "\r\n" + "Paolo Paglianti" + "\r\n" + "\r\n" + "PRODUCTION DESIGN" + "\r\n" + "Adriana Bienati" + "\r\n" + "\r\n" + "MARKETING EXECUTIVE" + "\r\n" + "Roberta Migliori" + "\r\n" + "\r\n" + "LEAD ARTIST" + "\r\n" + "Pat Ward" + "\r\n" + "\r\n" + "ARTIST" + "\r\n" + "Koen Bekkema" + "\r\n" + "\r\n" + "MANUAL LAYOUT" + "\r\n" + "Myriam Bell" + "\r\n" + "\r\n" + "PRODUCTION LEAD" + "\r\n" + "Matthew Ravenwood" + "\r\n" + "\r\n" + "PRODUCTION TEAM" + "\r\n" + "Sam O’Neill, Joseph Stephenson" + "\r\n" + "\r\n" + "ADMINISTRATION" + "\r\n" + "Dean Walker" + "\r\n" + "\r\n" + "ADMINISTRATION ASSISTANT" + "\r\n" + "Richard Baker" + "\r\n" + "\r\n" + "CUSTOMER SUPPORT STAFF" + "\r\n" + "Paulo Costa, Joseph Miller" + "\r\n" + "\r\n" + "WEB DEVELOPMENT" + "\r\n" + "Valery Vidershpan, Andrea Nicola, Fernando Turi" + "\r\n" + "\r\n";
      ref Graphics local1 = ref g;
      bitmap1: Bitmap = BitmapStore.GetBitmap(this.game.MGSPLASH);
      ref local2: Bitmap = ref bitmap1;
      DrawMod.DrawScaled(ref local1, ref local2, 595, 30, 130, 130);
      let mut tsubpart1: SubPartClass =  new TextAreaClass2(this.game, 380, 25, this.game.MarcFont4, tText2, tbackbitmap: (ref this.BackBitmap), bbx: 470, bby: 156, tcenterit: true);
      this.Ta2id = this.AddSubPart(ref tsubpart1, 470, 156, 380, 416, 0);
      ref Graphics local3 = ref g;
      bitmap2: Bitmap = BitmapStore.GetBitmap(this.game.VRSPLASH);
      ref local4: Bitmap = ref bitmap2;
      DrawMod.DrawScaled(ref local3, ref local4, 155, 30, 130, 130);
      let mut tsubpart2: SubPartClass =  new TextAreaClass2(this.game, 380, 25, this.game.MarcFont4, tText1, tbackbitmap: (ref this.BackBitmap), bbx: 30, bby: 156, tcenterit: true);
      this.TAid = this.AddSubPart(ref tsubpart2, 30, 156, 380, 416, 0);
      let mut tsubpart3: SubPartClass =  new TextButtonPartClass("OK", 150, "Click to return to main screen. [ESC]", ref this.OwnBitmap, 365, 620, theight: 40, usefont: this.game.MarcFont4, useshadow: true, tMarcStyle: true);
      this.cancelid = this.AddSubPart(ref tsubpart3, 365, 620, 150, 40, 1);
    }

    pub fn View2()
    {
      Graphics objGraphics = Graphics.FromImage((Image) this.OwnBitmap);
      this.tx += 101;
      if (this.tx > 870)
      {
        this.tx = 0;
        this.ty += 101;
      }
      if (this.ty > 600)
      {
        this.ty = 0;
        this.tr -= 25;
      }
      if (this.tr < 1)
        this.tr =  byte.MaxValue;
      DrawMod.DrawBlock(ref objGraphics, this.tx, this.ty, 100, 100, this.tr,  byte.MaxValue,  byte.MaxValue,  byte.MaxValue);
    }

    pub HandleKeyPress: WindowReturnClass(nr: i32, bool fromTimer = false)
    {
      windowReturnClass: WindowReturnClass = WindowReturnClass::new();
      try
      {
        if (nr == 27 | nr == 32)
        {
          windowReturnClass.AddCommand(6, 0);
          windowReturnClass.SetFlag(true);
          return windowReturnClass;
        }
      }
      catch (Exception ex)
      {
        ProjectData.SetProjectError(ex);
        ProjectData.ClearProjectError();
      }
      return windowReturnClass;
    }

    pub HandleMouseClick: WindowReturnClass(x: i32, y: i32, b: i32)
    {
      windowReturnClass: WindowReturnClass = WindowReturnClass::new();
      if (this.SubPartCounter > -1)
      {
        let mut subPartCounter: i32 =  this.SubPartCounter;
        for (let mut index: i32 =  0; index <= subPartCounter; index += 1)
        {
          if (x > this.SubPartX[index] & x < this.SubPartX[index] + this.SubPartW[index] && y > this.SubPartY[index] & y < this.SubPartY[index] + this.SubPartH[index])
          {
            let mut num: i32 =  this.SubPartID[index];
            if (num == this.TAid)
            {
              this.SubPartList[index].Click(x - this.SubPartX[index], y - this.SubPartY[index]);
              this.SubPartFlag[index] = true;
              windowReturnClass.SetFlag(true);
              return windowReturnClass;
            }
            if (num == this.Ta2id)
            {
              this.SubPartList[index].Click(x - this.SubPartX[index], y - this.SubPartY[index]);
              this.SubPartFlag[index] = true;
              windowReturnClass.SetFlag(true);
              return windowReturnClass;
            }
            if (num == this.cancelid)
            {
              this.SubPartList[index].Click(x - this.SubPartX[index], y - this.SubPartY[index]);
              windowReturnClass.AddCommand(6, 0);
              windowReturnClass.SetFlag(true);
              return windowReturnClass;
            }
          }
        }
        windowReturnClass.SetFlag(false);
        return windowReturnClass;
      }
      windowReturnClass.SetFlag(false);
      return windowReturnClass;
    }
  }
}
