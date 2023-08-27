// Decompiled with JetBrains decompiler
// Type: WindowsApplication1.ATCreditsInfoWindowClass
// Assembly: WindowsApplication1, Version=1.0.8020.28903, Culture=neutral, PublicKeyToken=null
// MVID: F52869E5-0850-48AD-BBBE-68E7A4900AFE
// Assembly location: C:\Program Files (x86)\Steam\steamapps\common\Shadow Empire\ShadowEmpire.exe

// usingMicrosoft.VisualBasic;
// usingMicrosoft.VisualBasic.CompilerServices;
// usingSystem;
// usingSystem.Drawing;

namespace WindowsApplication1
{
  pub class ATCreditsInfoWindowClass : WindowClass
  {
     int okid;
     int cancelid;
     int oktextid;
     int Pic1Id;
     int TAid;
     int Ta2id;
     int His;
     int Card;
     int Unr;

    pub ATCreditsInfoWindowClass(ref tGame: GameClass)
      : base(ref tGame, 880, 680, BackSprite: tGame.BACKGROUND2MARC, tBackSpriteScaled: true)
    {
      this.View();
    }

    pub fn HandleToolTip(int x, int y)
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
      this.NewBackGroundAndClearAll(880, 680, this.game.BACKGROUND2MARC);
      Graphics objgraphics = Graphics.FromImage((Image) this.OwnBitmap);
      this.BackBitmap = (Bitmap) this.OwnBitmap.Clone();
      DrawMod.DrawTextVic2(ref objgraphics, "CREDITS", Font::new("Arial", 54f, FontStyle.Underline, GraphicsUnit.Pixel), 300, 65, Color.FromArgb(128, 0, 0, 0), Color.FromArgb(128, 0, 0, 0));
      DrawMod.DrawTextVic2(ref objgraphics, "CREDITS", Font::new("Arial", 54f, FontStyle.Underline, GraphicsUnit.Pixel), 300, 60, Color.White, Color.FromArgb(128, 0, 0, 0));
      ref Graphics local1 = ref objgraphics;
      bitmap1: Bitmap = BitmapStore.GetBitmap(this.game.VRSPLASH);
      ref local2: Bitmap = ref bitmap1;
      DrawMod.DrawScaled(ref local1, ref local2, 168, 30, 110, 110);
      str1: String;
      let mut tsubpart: SubPartClass =  new TextAreaClass(this.game, 380, 26, this.game.VicFont3, "", false, str1 + "\r\n" + "GAME DESIGN, PROGRAMMING" + "\r\n" + "Victor Reijkersz" + "\r\n" + "\r\n" + "HEX ART GRAPHICS" + "\r\n" + "Frédéric Genot" + "\r\n" + "\r\n" + "TROOPTYPE & LANDSCAPE ILLUSTRATIONS" + "\r\n" + "Steve Ford" + "\r\n" + "\r\n" + "COMMANDER PORTRAITS & ILLUSTRATIONS" + "\r\n" + "Mike Gaffney" + "\r\n" + "\r\n" + "SCENARIO DESIGN" + "\r\n" + "Victor Reijkersz, Tom Weber" + "\r\n" + "\r\n" + "EXTRA SCENARIO GRAPHICS" + "\r\n" + "Stuart Zastrow" + "\r\n" + "\r\n" + "MANUAL, TUTORIAL AND WIKI WRITING" + "\r\n" + "Victor Reijkersz, Roy Blackwell, Eric Heiser" + "\r\n" + "\r\n" + "INTERNAL VR DESIGNS TESTING" + "\r\n" + "'Lunaticus', Juergen Stephan, Tom Weber, Howard Wakefield, Vance Strickland, Tomas Olvmyr, Roy Blackwell, Eric Heiser, Russ Arendell, Juergen Kraut" + "\r\n" + "\r\n", DrawMod.TGame.VicColor2, tbackbitmap: (ref this.BackBitmap), bbx: 45, bby: 156, tcenterit: true);
      this.TAid = this.AddSubPart(ref tsubpart, 45, 156, 380, 432, 0);
      ref Graphics local3 = ref objgraphics;
      bitmap2: Bitmap = BitmapStore.GetBitmap(this.game.MGSPLASH);
      ref local4: Bitmap = ref bitmap2;
      DrawMod.DrawScaled(ref local3, ref local4, 590, 30, 110, 110);
      str2: String;
      tsubpart =  new TextAreaClass(this.game, 380, 26, this.game.VicFont3, "", false, str2 + "\r\n" + "EXECUTIVE PRODUCER " + "\r\n" + "David Heath " + "\r\n" + "\r\n" + "ASSOCIATE PRODUCER " + "\r\n" + "Erik Rutins " + "\r\n" + "\r\n" + "BOX DESIGN " + "\r\n" + "Marc von Martial " + "\r\n" + "\r\n" + "MANUAL EDITING AND CONTENT" + "\r\n" + "Shaun Wallace, Toby " + "\r\n" + "\r\n" + "MATRIX TESTERS" + "\r\n" + "John Duquette, Richard Giberson, Gary Bezant, Klaus Weichel, Matt Williams, Bill Wheatley, Alexandre Fenelon, Allan Anderson" + "\r\n" + "\r\n" + "MANUAL DESIGN AND LAYOUT" + "\r\n" + "Marc von Martial " + "\r\n" + "\r\n" + "PUBLIC RELATIONS & MARKETING " + "\r\n" + "Sean Drummy " + "\r\n" + "\r\n" + "PRODUCTION ASSISTANT " + "\r\n" + "Andrew Loveridge " + "\r\n" + "\r\n" + "DIRECTOR OF OPERATIONS AND ACQUISITIONS" + "\r\n" + "David Heath " + "\r\n" + "\r\n" + "MANAGER OF DEALER RELATIONS & BUSINESS DEVELOPMENT" + "\r\n" + "Karlis Rutins" + "\r\n" + "\r\n" + "PRODUCTION ASSISTANT " + "\r\n" + "Andrew Loveridge " + "\r\n" + "\r\n" + "LEAD ADMINISTRATION  " + "\r\n" + "Liz Stoltz " + "\r\n" + "\r\n" + "DISTRIBUTOR SALES MANAGER " + "\r\n" + "Ross Jepson " + "\r\n" + "\r\n" + "SERIOUS GAMES PROJECT MANAGER " + "\r\n" + "Shaun Wallace, David Heath " + "\r\n" + "\r\n" + "CUSTOMER SUPPORT STAFF " + "\r\n" + "Daniel Heath, Alex Fiedler " + "\r\n" + "\r\n" + "FORUM ADMINISTRATION " + "\r\n" + "Alex Fiedler, Marc von Martial, Erik Rutins , David Heath" + "\r\n" + "\r\n" + "WEB-DATABASE DESIGN & DEVELOPMENT " + "\r\n" + "Alex Fiedler " + "\r\n" + "\r\n" + "NETWORK AND SYSTEM ADMINISTRATOR " + "\r\n" + "Alex Fiedler, David Heath" + "\r\n" + "\r\n" + "SOCIAL MEDIA MANAGER" + "\r\n" + "Steve Stafford" + "\r\n" + "\r\n" + "PC SUPPORT AND NETWORK ADMINISTRATOR" + "\r\n" + "Lance Stoltz " + "\r\n" + "\r\n" + "QUALITY ASSURANCE LEAD " + "\r\n" + "Erik Rutins " + "\r\n" + "\r\n" + "VERY SPECIAL THANKS " + "\r\n" + "To our new colleagues at Slitherine Ltd.: JD McNeil, Ian McNeil, Marco Minoli, Philip Veale, Andrea Nicola, Richard Evans, Christian Bassani." + "\r\n" + "\r\n" + "MATRIX NEXGEN " + "\r\n" + "Alexander Rutins, Andrew Heath, Nicholas Heath, Shane Heath, Austin Stoltz, Noah Stoltz, Jesse Stoltz, Heidi Fiedler, Blake Fiedler, Harold Dupree" + "\r\n" + "\r\n" + "OUR STRENGTH " + "\r\n" + "We thank God for giving us the ability and strength to complete this project and follow our dream. " + "We would also like to thank our families and friends for giving us their non-stop love and support during this project.", DrawMod.TGame.VicColor2, tbackbitmap: (ref this.BackBitmap), bbx: 455, bby: 156, tcenterit: true);
      this.Ta2id = this.AddSubPart(ref tsubpart, 455, 156, 380, 432, 0);
      tsubpart =  new TextButtonPartClass("OK", 150, "Click to return to main screen. [ESC]", ref this.OwnBitmap, 350, 620, theight: 40);
      this.cancelid = this.AddSubPart(ref tsubpart, 350, 620, 150, 40, 1);
      if (Information.IsNothing( objgraphics))
        return;
      objgraphics.Dispose();
      objgraphics = (Graphics) null;
    }

    pub HandleKeyPress: WindowReturnClass(int nr, bool fromTimer = false)
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

    pub HandleMouseClick: WindowReturnClass(int x, int y, int b)
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
