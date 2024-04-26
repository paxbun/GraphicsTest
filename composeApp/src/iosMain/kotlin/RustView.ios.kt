import androidx.compose.runtime.Composable
import androidx.compose.ui.Modifier
import androidx.compose.ui.interop.UIKitView
import graphicsTest.RustNativeViewLib_nativeViewAsNativeHandle
import graphics_test.RustNativeViewContext
import kotlinx.cinterop.BetaInteropApi
import kotlinx.cinterop.CPointed
import kotlinx.cinterop.CValue
import kotlinx.cinterop.ExperimentalForeignApi
import kotlinx.cinterop.memScoped
import kotlinx.cinterop.objcPtr
import kotlinx.cinterop.readValue
import kotlinx.cinterop.toCPointer
import kotlinx.cinterop.useContents
import platform.CoreGraphics.CGRect
import platform.CoreGraphics.CGRectNull
import platform.Foundation.NSCoder
import platform.QuartzCore.CAMetalLayer
import platform.UIKit.UIScreen
import platform.UIKit.UIView
import platform.UIKit.UIViewMeta
import kotlin.experimental.ExperimentalObjCName

@OptIn(ExperimentalForeignApi::class)
@Composable
actual fun RustView(modifier: Modifier) {
    UIKitView(
        factory = { RustNativeView() },
        modifier = modifier,
        update = RustNativeView::render,
        onRelease = RustNativeView::destroy,
    )
}

@OptIn(ExperimentalObjCName::class, ExperimentalForeignApi::class, BetaInteropApi::class)
private class RustNativeView : UIView {
    private lateinit var nativeViewContext: RustNativeViewContext

    companion object : UIViewMeta() {
        override fun layerClass() = CAMetalLayer
    }

    @Suppress("UNUSED") // required by Objective-C runtime
    @OverrideInit
    constructor(coder: NSCoder) : super(coder) {
        throw UnsupportedOperationException("init(coder: NSCoder) is not supported for RustNativeView")
    }


    @OverrideInit
    constructor(frame: CValue<CGRect> = CGRectNull.readValue()) : super(frame)

    override fun layoutSubviews() {
        super.layoutSubviews()
        if (!::nativeViewContext.isInitialized) {
            nativeViewContext = RustNativeViewContext(
                RustNativeViewLib_nativeViewAsNativeHandle(
                    objcPtr().toLong().toCPointer<CPointed>()
                ).apply {
                    if (this == 0L) {
                        error("Could not create RustNativeViewContext")
                    }
                },
                UIScreen.mainScreen.scale.toFloat(),
            )
        } else {
            memScoped {
                val contentsScale = layer.contentsScale
                val size = layer.bounds.useContents { size }
                if (size.width != 0.0 && size.height != 0.0) {
                    nativeViewContext.changeSize(
                        (size.width * contentsScale).toInt(),
                        (size.height * contentsScale).toInt(),
                        contentsScale.toFloat(),
                    )
                }
            }
        }
        nativeViewContext.render()
    }

    override fun didMoveToWindow() {
        println("didMoveToWindow")
        window?.windowScene?.screen?.scale?.toFloat()
    }

    fun render() {
        if (::nativeViewContext.isInitialized) {
            nativeViewContext.render()
        }
    }

    fun destroy() {
        if (::nativeViewContext.isInitialized) {
            nativeViewContext.destroy()
        }
    }
}